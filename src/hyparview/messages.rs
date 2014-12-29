use std::io::{IoError,IoErrorKind,IoResult};
use std::io::net::ip::{Ipv4Addr,SocketAddr};
use std::io::net::tcp::{TcpStream};

// TODO: need a much better system for identifying the messages (by type) than this simple hard-coded list, but wtf...
static HPV_MSG_ID_JOIN: u8 = 0;
static HPV_MSG_ID_FORWARD_JOIN: u8 = 1;
static HPV_MSG_ID_JOIN_ACK: u8 = 2;
static HPV_MSG_ID_DISCONNECT: u8 = 3;

// enum Priority {
//     High,
//     Low
// }

#[deriving(Copy,Show)]
pub enum HyParViewMessage {
    JoinMessage(Join,SocketAddr),
    ForwardJoinMessage(ForwardJoin,SocketAddr),
    JoinAckMessage(JoinAck,SocketAddr),
    DisconnectMessage(Disconnect,SocketAddr),
}

/// top-level function for serializing a HyParView message.
pub fn deserialize(reader: &mut TcpStream) -> IoResult<HyParViewMessage> {
    let addr = reader.peer_name().ok().expect("failed to get the remote peer addr from an open socket.");
    match reader.read_u8() {
        Ok(0) => Ok(HyParViewMessage::JoinMessage(Join::new(), addr)),
        Ok(1) => Ok(HyParViewMessage::ForwardJoinMessage(ForwardJoin::deserialize(reader).ok().expect("failed to deserailize the forward join"), addr)),
        Ok(2) => Ok(HyParViewMessage::JoinAckMessage(JoinAck::new(), addr)),
        Ok(3) => Ok(HyParViewMessage::DisconnectMessage(Disconnect::new(), addr)),
        Err(e) => Err(e),
        _ => Err(IoError{ kind: IoErrorKind::InvalidInput, desc: "unknown message id passed in".as_slice(), detail: None }),
    }
}

/// helper function to efficiently serialize a SocketAddr
fn serialize_socket_addr(sa: &SocketAddr, writer: &mut Writer) -> IoResult<int> {
    match sa.ip {
        Ipv4Addr(a, b, c, d) => {
            writer.write_u8(a).ok();
            writer.write_u8(b).ok();
            writer.write_u8(c).ok();
            writer.write_u8(d).ok();
            writer.write_be_u16(sa.port).ok();
        },
        _ => println!("dont care yet!"),
    }
    Ok(4 + 2)
}

/// helper function to efficiently deserialize a SocketAddr
fn deserialize_socket_addr(reader: &mut Reader) -> IoResult<SocketAddr> {
    let mut buf = [0u8, ..4];
    let mut i = 0;
    while i < buf.len() {
        buf[i] = reader.read_u8().ok().expect("couldn't read next byte for ip address");
        i += 1;
    }
    let ip = Ipv4Addr (buf[0], buf[1], buf[2], buf[3]);
    let port = reader.read_be_u16().ok().expect("couldn't read port for socket address");

    let sa: SocketAddr = SocketAddr { ip: ip , port: port };
    Ok(sa)
}

#[deriving(Copy,Show)]
//TODO: add a message uuid so we can register a callback (to make sure the join message gets a reponse, else resend the request)
pub struct Join;
impl Join {
    pub fn new() -> Join {
        Join
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_JOIN).ok();
        Ok(1)
    }
}

#[deriving(Copy,Show)]
pub struct ForwardJoin {
    pub originator: SocketAddr,
    pub arwl: u8,
    pub prwl: u8,
    pub ttl: u8
}
impl ForwardJoin {
    pub fn new(addr: &SocketAddr, arwl: u8, prwl: u8, ttl: u8) -> ForwardJoin {
        ForwardJoin { originator: *addr, arwl: arwl, prwl: prwl, ttl: ttl }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<ForwardJoin> {
        match deserialize_socket_addr(reader) {
            Ok(socket) => {
                let arwl = reader.read_u8().ok().expect("could not read arwl from stream"); 
                let prwl = reader.read_u8().ok().expect("could not read prwl from stream"); 
                let ttl = reader.read_u8().ok().expect("could not read ttl from stream"); 
                Ok(ForwardJoin::new(&socket, arwl, prwl, ttl))
            },
            Err(e) => Err(e),
        }
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_FORWARD_JOIN).ok();
        let mut cnt = 1;

        match serialize_socket_addr(&self.originator, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }

        writer.write_u8(self.arwl).ok();
        cnt += 1;
        writer.write_u8(self.prwl).ok();
        cnt += 1;
        writer.write_u8(self.ttl).ok();
        cnt += 1;
        Ok(cnt)
    }
}

#[test]
fn test_join_serialization() {
    use std::io::{MemReader,BufferedWriter};
    use std::io::net::ip::{SocketAddr};

    let arwl = 6u8;
    let prwl = 3u8;
    let ttl = 4u8;

    let sock_addr: SocketAddr = from_str("127.0.0.1:9090").expect("invalid socket addr");
    let fjoin_msg = ForwardJoin::new(&sock_addr, arwl, prwl, ttl);
    let vec = Vec::new();
    let mut writer = BufferedWriter::new(vec);
    let result = fjoin_msg.serialize(&mut writer);
    
    let vec = writer.into_inner();
    let mut reader = MemReader::new(vec);
    let header = reader.read_u8().ok().expect("failed to read the header");
    let return_fjoin_msg = ForwardJoin::deserialize(&mut reader).ok().expect("failed to parse socket addr");
    assert!(return_fjoin_msg.originator.eq(&sock_addr));
    assert_eq!(return_fjoin_msg.arwl, arwl);
    assert_eq!(return_fjoin_msg.prwl, prwl);
    assert_eq!(return_fjoin_msg.ttl, ttl);
}

#[deriving(Copy,Show)]
pub struct Disconnect;
impl Disconnect {
    pub fn new() -> Disconnect {
        Disconnect
    }

     pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_DISCONNECT).ok();
        Ok(1)
    }
}

#[deriving(Copy,Show)]
pub struct JoinAck;
impl JoinAck {
    pub fn new() -> JoinAck {
        JoinAck
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_JOIN_ACK).ok();
        Ok(1)
    }
}

// pub struct Neighbor {
//     sender: SocketAddr,
//     priority: Priority
// }

// pub struct NeighborReject {
//     sender: SocketAddr,
// }
