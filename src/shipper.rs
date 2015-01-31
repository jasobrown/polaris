use hyparview::messages::{Serializable};
use std::old_io::net::ip::SocketAddr;
use std::old_io::net::tcp::TcpStream;
use std::time::Duration;

pub trait Shipper {
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool;
}

pub struct SocketShipper {
    pub local_addr: SocketAddr,
}
impl Shipper for SocketShipper {
    fn ship(&self, msg: &Serializable, dest: &SocketAddr) -> bool {
        let mut success = false;
        match TcpStream::connect_timeout(*dest, Duration::seconds(4)) {
            Ok(ref mut socket) => {
                match msg.serialize(&mut* socket, &self.local_addr) {
                    Ok(_) => success = true,
                    Err(e) => warn!("failed to send message to peer {}: {}", dest, e),
                }
            },
            Err(e) => warn!("failed to open connection to peer (to forward the shuffle): {}", e),
        }
        success
    }
}
