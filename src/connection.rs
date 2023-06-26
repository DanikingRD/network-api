use std::net::{SocketAddr, Ipv4Addr};
use std::io::Error;

use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::net::UdpSocket;
use crate::bind_udp_socket;

pub struct Connection {
    sock: UdpSocket,
}

const PACKET_BUFFER_SIZE: usize = 1024;

impl Connection { 

    pub async fn connect(addr: SocketAddr) -> Result<Self, Error> {
        let this = Self::bind_to_any()?;
        this.sock.connect(addr).await?;
        Ok(this)
    }

    pub fn bind_to_any() -> Result<Self, Error> {
        Self::bind_to(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
    }

    pub fn bind_to(addr: SocketAddr) -> Result<Self, Error> {
        Ok( Self {
            sock: bind_udp_socket(addr)?,
        })
    }

    pub async fn send<P: Serialize>(&self, packet: P)  {
        let buffer = Self::serialize(&packet);
        self.sock.send(&buffer).await.expect("Failed to send packet");
    }

    pub async fn recv<P: DeserializeOwned>(&self) -> Result<P, Error> {
        let mut buffer = [0u8; PACKET_BUFFER_SIZE];
        match self.sock.recv(&mut buffer).await {
            Ok(len) => Self::deserialize::<P>(&buffer[0..len]),
            Err(e) => Err(e),
        }
    }

    pub async fn peek<P: DeserializeOwned>(&self) -> Result<(P, SocketAddr), Error> {
        let mut buffer = [0u8; PACKET_BUFFER_SIZE];
        match self.sock.peek_from(&mut buffer).await {
            Ok((len, addr)) => Self::deserialize::<P>(&buffer[0..len]).map(|p| (p, addr)),
            Err(e) => Err(e),
        }
    }

    pub fn serialize<P: Serialize>(p: &P) -> Vec<u8> {
        bincode::serialize(p).unwrap()
    }

    pub fn deserialize<P: DeserializeOwned>(bytes: &[u8]) -> Result<P, std::io::Error> {
        return match bincode::deserialize::<P>(&bytes) {
            Ok(t) => Ok(t),
            Err(e) => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            },
        };
    }

    pub fn addr(&self) -> SocketAddr {
        self.sock.local_addr().unwrap()
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.sock.peer_addr().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::ClientMessage;

    use super::*;

    #[tokio::test]
    async fn test_constructor() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = Connection::bind_to(addr).unwrap();
        let client = Connection::connect(server.addr()).await.unwrap();
        assert_eq!(server.addr(), client.peer_addr());
    }

    #[tokio::test]
    async fn send_recv() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = Connection::bind_to(addr).unwrap();
        let client = Connection::connect(server.addr()).await.unwrap();

        client.send(ClientMessage::Connect).await;
        let packet = server.recv::<ClientMessage>().await.unwrap();
        assert!(matches!(packet, ClientMessage::Connect));
    }

    #[tokio::test]
    async fn send_peek_recv() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = Connection::bind_to(addr).unwrap();
        let client = Connection::connect(server.addr()).await.unwrap();

        client.send(ClientMessage::Connect).await;
        let (packet, _) = server.peek::<ClientMessage>().await.unwrap();
        assert!(matches!(packet, ClientMessage::Connect));

        let packet = server.recv::<ClientMessage>().await.unwrap();
        assert!(matches!(packet, ClientMessage::Connect));
    }
}