use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use bincode::{config};
use bytes::Bytes;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use webrtc::api::{API, APIBuilder};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::data_channel::RTCDataChannel;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;

pub mod model;

#[post("/connect")]
async fn connect(
	api: web::Data<API>,
	clients: web::Data<model::WebRtcClients>,
	body: web::Json<model::ConnectRequest>
) -> impl Responder {
	log::info!("connect request: {:?}", body);

	// Create peer_connection
	let peer_connection = match api.new_peer_connection(RTCConfiguration::default()).await {
		Ok(p) => p,
		Err(err) => panic!("{}", err),
	};
	let peer_connection = Arc::new(peer_connection);

	peer_connection.on_peer_connection_state_change(Box::new(
		|connection_state: RTCPeerConnectionState| {
			log::info!("Peer Connection State has changed: {}", connection_state);
			Box::pin(async {})
		},
	));

	let client_id = body.id;
	let clients_state = clients.clone();
	peer_connection.on_ice_connection_state_change(Box::new(
		move |connection_state: RTCIceConnectionState| {
			log::info!("ICE Connection State has changed: {}", &connection_state);
			let clients_state_2 = clients_state.clone();
			Box::pin(async move {
				match connection_state {
					RTCIceConnectionState::Disconnected => {
						let mut connections = clients_state_2.connections.write().await;
						(*connections).remove(&client_id);
					},
					_ => log::info!("No action"),
				};
			})
		},
	));

	peer_connection.on_data_channel(Box::new(
		move |data_channel: Arc<RTCDataChannel>| {
			log::info!("New Datachannel from user {}: {}", &client_id, &data_channel.label());

			// Create message handler
			let dc_copy = data_channel.clone();
			data_channel.on_message(Box::new(
				move |msg: DataChannelMessage| {
					let config = config::standard();
					let (msg_str, _len): (String, usize) = match bincode::decode_from_slice(&msg.data, config) {
						Ok(d) => d,
						Err(e) => panic!("{}", e),
					};
					//let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
					log::info!("Message from DataChannel '{}': '{}'", &dc_copy.label(), msg_str);

					let dc_reply = dc_copy.clone();
					Box::pin(async move {
						let chunk = model::Chunk::new();
						let config = config::standard();
						let payload = match bincode::encode_to_vec(&chunk, config) {
							Ok(p) => p,
							Err(e) => panic!("{}", e),
						};
						let payload = Bytes::from(payload);
						if let Err(e) = dc_reply.send(&payload).await {
							panic!("{}", e);
						}
					})
				}
			));

			Box::pin(async move {})
		},
	));

	// Connect to peer
	if let Err(err) = peer_connection.set_remote_description(body.webrtc_config.clone()).await {
		panic!("{}", err);
	}

	let mut gather_complete = peer_connection.gathering_complete_promise().await;

	let answer = match peer_connection.create_answer(None).await {
		Ok(answer) => answer,
		Err(err) => panic!("{}", err),
	};

	if let Err(err) = peer_connection.set_local_description(answer).await {
		panic!("{}", err);
	}

	let _ = gather_complete.recv().await;

	let payload = if let Some(local_desc) = peer_connection.local_description().await {
		match serde_json::to_string(&local_desc) {
			Ok(p) => p,
			Err(err) => panic!("{}", err),
		}
	} else {
		panic!("generate local_description failed!");
	};

	// Save peer_coonection
	let mut connections = clients.connections.write().await;
	(*connections).insert(body.id, peer_connection.clone());

	// Reply with connection config
	HttpResponse::Ok()
		.content_type(ContentType::json())
		.body(payload)
}

#[get("/clients")]
async fn get_clients(clients: web::Data<model::WebRtcClients>) -> impl Responder {
	let connections = clients.connections.read().await;
	let mut connections_vec = Vec::new();

	for (id, _) in (*connections).iter() {
		connections_vec.push(model::Connection {
			id: *id,
		});
	}

	let connections_json = serde_json::to_string(&connections_vec).unwrap();
	HttpResponse::Ok().body(connections_json)
}

async fn create_webrtc_app() -> API {
	let mut m = MediaEngine::default();
	m.register_default_codecs().unwrap();

	let mut registry = Registry::new();
	registry = register_default_interceptors(registry, &mut m).unwrap();

	APIBuilder::new()
		.with_media_engine(m)
		.with_interceptor_registry(registry)
		.build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

	log::info!("Server Started");

	let clients = web::Data::new(model::WebRtcClients {
		connections: RwLock::new(HashMap::new()),
	});

	let api = web::Data::new(create_webrtc_app().await);

	HttpServer::new(move || {
		App::new()
			.app_data(clients.clone())
			.app_data(api.clone())
			.service(connect)
			.service(get_clients)
			.wrap(Logger::default())
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
