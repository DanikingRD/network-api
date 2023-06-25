use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ClientMessage {
    Connect,
    Disconnect,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerMessage {
    Connected,
    Disconnected,
}