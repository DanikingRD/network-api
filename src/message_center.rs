use std::net::{SocketAddr};

use tokio::{net::UdpSocket, runtime::Runtime};
use std::io::Error;
use crate::{bind_udp_socket, connection::Connection, messages::ClientMessage};

pub struct MessageCenter {
    con: Connection,
}

impl MessageCenter {

    pub fn new(addr: SocketAddr) -> Self {
        let con = Connection::bind_to(addr).unwrap();
        Self {
            con,
        }
    }

    pub async fn accept(&self) -> impl ExactSizeIterator<Item = Connection> {
        let mut cons = Vec::new();
        loop {
            // TODO: break when no more connections
            match self.con.peek::<ClientMessage>().await {
                Ok((ClientMessage::Connect, addr)) => {
                    let con = Connection::connect(addr).await.unwrap();
                    cons.push(con);
                },
                Ok(_) => continue,
                Err(e) => break,
            }
        }
        cons.into_iter()
    }   
}

async fn handle_incoming() -> impl ExactSizeIterator<Item = MessageHandler> {
    let handlers = vec![];

    handlers.into_iter()
}

pub struct MessageHandler {}

impl MessageHandler {
    pub fn new() -> Self {
        tokio::spawn(async move {});

        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::ClientMessage;

    use super::*;

    #[tokio::test]
    async fn test_constructor() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = MessageCenter::new(addr);
        let client = Connection::connect(addr).await.unwrap();

        for _ in 0..10 {
            client.send(ClientMessage::Connect).await;
        }

        server.accept();
    }   
}