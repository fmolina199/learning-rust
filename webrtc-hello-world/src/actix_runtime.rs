use awc::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{Duration, Instant, sleep};
use webrtc::api::APIBuilder;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRequest {
    id: u64,
    webrtc_config: RTCSessionDescription,
}

fn main() {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

	let rt = actix_rt::Runtime::new().unwrap();

	let handle = rt.spawn(async {
		// Create WebRTC API
		let mut m = MediaEngine::default();
		m.register_default_codecs().unwrap();

		let mut registry = Registry::new();
		registry = register_default_interceptors(registry, &mut m).unwrap();

		let api = APIBuilder::new()
			.with_media_engine(m)
			.with_interceptor_registry(registry)
			.build();

		// Create peer connection
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

		peer_connection.on_ice_connection_state_change(Box::new(
			|connection_state: RTCIceConnectionState| {
				log::info!("ICE Connection State has changed: {}", connection_state);
				Box::pin(async {})
			},
		));

		let data_channel = match peer_connection.create_data_channel("data", None).await {
			Ok(dc) => dc,
			Err(err) => panic!("{}", err),
		};

		let dc_copy = Arc::clone(&data_channel);
		data_channel.on_open(Box::new(
			move || {
				log::info!("Data channel '{}'-'{}'", &dc_copy.label(), &dc_copy.id());
				let dc_send = dc_copy.clone();
				Box::pin(async move {
					log::info!("Sending Message");
					while dc_send
						.send_text(format!("{:?}", Instant::now()))
						.await
						.is_ok()
					{
						sleep(Duration::from_secs(3)).await;
					}
				})
			}
		));

		let d_label = data_channel.label().to_owned();
		data_channel.on_message(Box::new(
			move |msg: DataChannelMessage| {
				let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
				log::info!("Message from DataChannel '{}': '{}'", d_label, msg_str);
				Box::pin(async {})
			}
		));

		let mut gather_complete = peer_connection.gathering_complete_promise().await;

		let offer = match peer_connection.create_offer(None).await {
			Ok(offer) => offer,
			Err(err) => panic!("{}", err),
		};

		if let Err(err) = peer_connection.set_local_description(offer).await {
			panic!("{}", err);
		}

		let _ = gather_complete.recv().await;


		let payload = match peer_connection.local_description().await {
			Some(local_desc) => ConnectRequest {
				id: rand::random::<u64>(),
				webrtc_config: local_desc
			},
			None => panic!("generate local_description failed!"),
		};

		// Send request
		let client = Client::default();

		let mut res = match client.post("http://fmolina.com.br:8080/connect")
			.insert_header(("User-Agent", "rust-awc/3"))
			.send_json(&payload)
			.await
		{
			Ok(res) => res,
			Err(err) => panic!("{}", err),
		};

		log::info!("Response: {:?}", res);

		let val = match res.json::<RTCSessionDescription>().await {
			Ok(val) => val,
			Err(err) => panic!("{}", err),
		};

		if let Err(err) = peer_connection.set_remote_description(val).await {
			panic!("{}", err);
		}

		sleep(Duration::from_secs(300)).await;
	});

	rt.block_on(handle).unwrap()
}
