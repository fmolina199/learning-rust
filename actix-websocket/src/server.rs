use std::collections::HashMap;
use actix_broker::BrokerSubscribe;

use actix::prelude::*;

use crate::messages::*;

pub struct Client {
	pub name: Option<String>,
	pub recipient: Recipient<StrMessage>,
}

#[derive(Default)]
pub struct WsChatServer {
	clients: HashMap<u64, Client>,
}

impl Actor for WsChatServer {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		log::info!("WsChatServer started");
		self.subscribe_system_async::<Unregister>(ctx);
		//self.subscribe_system_async::<SendMessage>(ctx);
	}   

	fn stopped(&mut self, _ctx: &mut Self::Context) {
		log::info!("WsChatServer stopped");
	}
}

impl Handler<Register> for WsChatServer {
	type Result = MessageResult<Register>;

	fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
		log::info!("Register: new client");

		let Register(recipient) = msg;
		let mut id = rand::random::<u64>();
		loop {
			if self.clients.contains_key(&id) {
				id = rand::random::<u64>();
			} else {
				break;
			}
		}

		self.clients.insert(id, Client { name: None, recipient: recipient });
		MessageResult(id)
	}
}

impl Handler<Unregister> for WsChatServer {
	type Result = MessageResult<Unregister>;

	fn handle(&mut self, msg: Unregister, _ctx: &mut Self::Context) -> Self::Result {
		let Unregister(id) = msg;
		log::info!("Unregister: client {}", id);

		if self.clients.contains_key(&id) {
			self.clients.remove(&id);
		}
		MessageResult(())
	}
}

impl Handler<GetMap> for WsChatServer {
	type Result = ();

	fn handle(&mut self, msg: GetMap, _ctx: &mut Self::Context) {
		let GetMap(x, y) = msg;
		log::info!("GetMap: {{ x: {}, y: {} }}", x, y);
		for (id, recipient) in &self.clients {
			log::info!("id: {}", id);
		}
	}
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
