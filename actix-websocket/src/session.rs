use actix::{fut, prelude::*};
use actix_web_actors::ws;
use actix_broker::BrokerIssue;

use serde::{Serialize, Deserialize};

use crate::{
	messages::*,
	server::WsChatServer,
};

#[derive(Default)]
pub struct WsChatSession {
	id: u64,
	//name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct WsRequest {
	user_id: u64,
	friend_id: u64,
}

impl WsChatSession {
	pub fn get_map(&mut self, request: GetMap, ctx: &mut ws::WebsocketContext<Self>) {
		WsChatServer::from_registry()
			.send(request)
			.into_actor(self)
			.then(|res, _, ctx| {
				if let Ok(_res) = res {
					ctx.text("OK");
				} else {
					ctx.text("NOK");
				}
				fut::ready(())
			})
			.wait(ctx);
	}

	pub fn register(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
		let register_msg = Register(
			ctx.address().recipient(),
		);
		WsChatServer::from_registry()
			.send(register_msg)
			.into_actor(self)
			.then(|id, act, _ctx| {
				if let Ok(id) = id {
					act.id = id;
					log::info!("WsChatSession new session created: {}", &id);
				}
				fut::ready(())
			})
			.wait(ctx);
	}

	pub fn unregister(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
		let unregister_msg = Unregister(
			self.id,
		);
		self.issue_system_sync(unregister_msg, ctx);
		/*
		WsChatServer::from_registry()
			.send(unregister_msg)
			.into_actor(self)
			.then(|_id, act, _ctx| {
				log::info!("--> WsChatSession closed session");
				fut::ready(())
			})
			.wait(ctx);
		*/
	}
}

impl Actor for WsChatSession {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		log::info!("WsChatSession creating new session");
		self.register(ctx);
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		log::info!(
			"WsChatSession closing session for {}",
			self.id
		);
		self.unregister(ctx);
	}
}

impl Handler<StrMessage> for WsChatSession {
	type Result = ();

	fn handle(&mut self, msg: StrMessage, ctx: &mut Self::Context) {
		ctx.text(msg.0);
	}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		let msg = match msg {
			Err(_) => {
				ctx.stop();
				return;
			}
			Ok(msg) => msg,
		};

		log::debug!("WEBSOCKET MESSAGE: {msg:?}");

		match msg {
			ws::Message::Text(text) => {
				let req: Result<WsRequest, serde_json::Error> = serde_json::from_str(&text);
				match req {
					Err(error) => ctx.text(format!("Invalid Request: {:?}", error)),
					Ok(data) => self.get_map(GetMap(data.user_id, data.friend_id), ctx),
				};
			}
			ws::Message::Close(reason) => {
				ctx.close(reason);
				ctx.stop();
			}
			_ => {}
		}
	}
}
