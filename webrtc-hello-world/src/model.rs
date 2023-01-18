use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

#[derive(Default)]
pub struct WebRtcClients {
	pub connections: RwLock<HashMap<u64, Arc<RTCPeerConnection>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRequest {
	pub id: u64,
	pub webrtc_config: RTCSessionDescription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
	pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode, PartialEq)]
pub struct Chunk {
	pub map: [i32; 16],
	pub position: [i32; 3],
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk {
			map: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			position: [0, 0, 0],
		}
	}
}
