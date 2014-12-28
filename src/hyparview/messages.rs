use std::io::{IoResult};
use std::io::net::ip::{Ipv4Addr,SocketAddr};

// TODO: need a much better system for identifying the messages (by type) than this simple hard-coded list, but wtf...
static HPV_MSG_ID_JOIN: u8 = 0;
static HPV_MSG_ID_FORWARD_JOIN: u8 = 1;
//static HPV_MSG_ID_JOIN: u8 = 2;

// enum Priority {
//     High,
//     Low
// }

#[deriving(Copy,Show)]
pub enum HyParViewMessage {
    JoinMessage(Join),
    ForwardJoinMessage(ForwardJoin),
}

/// top-level function for serializing a HyParView message.
pub fn deserialize(reader: &mut Reader) -> IoResult<HyParViewMessage> {
    match reader.read_u8() {
        Ok(id) => Ok(HyParViewMessage::JoinMessage(Join::deserialize(reader).ok().expect("failed to deserailize the join"))),
        Err(e) => Err(e),
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
pub struct Join {
    sender:  SocketAddr,
    //TODO: add a message uuid so we can register a callback (to make sure the join message gets a reponse, else resend the request)
}
impl Join {
    pub fn new(sender: &SocketAddr) -> Join {
        Join { sender: *sender }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<Join> {
        match deserialize_socket_addr(reader) {
            Ok(socket) => Ok(Join::new(&socket)),
            Err(e) => Err(e),
        }
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_JOIN).ok();
        match serialize_socket_addr(&self.sender, writer) {
            Ok(cnt) => Ok(1 + cnt),
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test_join_serialization() {
    use std::io::{MemReader,BufferedWriter};
    use std::io::net::ip::{SocketAddr};

    let sock_addr: SocketAddr = from_str("127.0.0.1:9090").expect("invalid socket addr");
    let join_msg = Join::new(&sock_addr);
    let vec = Vec::new();
    let mut writer = BufferedWriter::new(vec);
    let result = join_msg.serialize(&mut writer);
    
    let vec = writer.into_inner();
    let mut reader = MemReader::new(vec);
    let return_join_msg = Join::deserialize(&mut reader).ok().expect("failed to parse socket addr");
    assert!(return_join_msg.sender.eq(&sock_addr));
}

#[deriving(Copy,Show)]
pub struct ForwardJoin {
    originator: SocketAddr,
    ttl: u8
}
impl ForwardJoin {
    pub fn new(addr: &SocketAddr, ttl: u8) -> ForwardJoin {
        ForwardJoin { originator: *addr, ttl: ttl }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<ForwardJoin> {
        match deserialize_socket_addr(reader) {
            Ok(socket) => {
                let ttl = reader.read_u8().ok().expect("could not read ttl from stream");
                Ok(ForwardJoin::new(&socket, ttl))
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

        writer.write_u8(self.ttl).ok();
        cnt += 1;
        Ok(cnt)
    }
}




// pub struct Disconncect {
//     sender: SocketAddr,
// }

// pub struct Neighbor {
//     sender: SocketAddr,
//     priority: Priority
// }

// pub struct NeighborReject {
//     sender: SocketAddr,
// }
