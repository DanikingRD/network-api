use std::net::{SocketAddr, Ipv4Addr};
use std::io::Error;

use tokio::net::UdpSocket;

use crate::bind_udp_socket;

pub struct Connection {
    sock: UdpSocket,
}

impl Connection{ 

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
            sock:  bind_udp_socket(addr)?,
        })
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

    #[tokio::test]
    async fn test_constructor() {
        use super::*;
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let server = Connection::bind_to(addr).unwrap();
        let client = Connection::connect(server.addr()).await.unwrap();
        assert_eq!(server.addr(), client.peer_addr());
    }
}