pub mod server;
pub mod client;
pub mod messages;
pub mod message_center;
pub mod connection;

use std::net::SocketAddr;

use server::Server;
use socket2::{Socket, Domain, Type, Protocol};
use tokio::net::UdpSocket;


fn main() {
    let server = Server::new();
}


pub fn bind_udp_socket(addr: SocketAddr) -> std::io::Result<UdpSocket> {
    let domain = Domain::for_address(addr);
    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_nonblocking(true)?;
    let socket2_addr = addr.into();
    socket.bind(&socket2_addr)?;
    let udp_socket = socket.into();
    let socket = UdpSocket::from_std(udp_socket)?;
    Ok(socket)
}