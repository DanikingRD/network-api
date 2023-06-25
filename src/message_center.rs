use std::net::{SocketAddr};


use tokio::{net::UdpSocket, runtime::Runtime};
use std::io::Error;
use crate::bind_udp_socket;

pub struct MessageCenter {
    socket: UdpSocket,
}

impl MessageCenter {

  

    /// Receive incoming messages
    pub fn incoming(&self, rt: &Runtime) -> impl ExactSizeIterator<Item = MessageHandler> {
        let handle = tokio::spawn(async move { handle_incoming().await });
        rt.block_on(handle).unwrap()
    }
}

async fn handle_incoming() -> impl ExactSizeIterator<Item = MessageHandler> {
    let handlers = vec![];

    handlers.into_iter()
}

pub struct MessageHandler {
    
}

impl MessageHandler {
    pub fn new() -> Self {
        tokio::spawn(async move {});

        Self {}
    }
}