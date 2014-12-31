use std::io::{IoError,IoErrorKind,IoResult};
use std::io::net::ip::{Ipv4Addr,SocketAddr};
use std::io::net::tcp::{TcpStream};

// TODO: need a much better system for identifying the messages (by type) than this simple hard-coded list, but wtf...
static HPV_MSG_ID_JOIN: u8 = 0;
static HPV_MSG_ID_FORWARD_JOIN: u8 = 1;
static HPV_MSG_ID_JOIN_ACK: u8 = 2;
static HPV_MSG_ID_DISCONNECT: u8 = 3;
static HPV_MSG_ID_NEIGHBOR_REQUEST: u8 = 4;
static HPV_MSG_ID_NEIGHBOR_RESPONSE: u8 = 5;
static HPV_MSG_ID_SHUFFLE: u8 = 6;
static HPV_MSG_ID_SHUFFLE_REPLY: u8 = 7;

#[deriving(Copy,Show,PartialEq)]
pub enum Priority {
    High,
    Low
}

#[deriving(Copy,Show)]
pub enum Result {
    Accept,
    Reject
}

#[deriving(Show)]
pub enum HyParViewMessage<'a> {
    JoinMessage(Join,SocketAddr),
    ForwardJoinMessage(ForwardJoin,SocketAddr),
    JoinAckMessage(JoinAck,SocketAddr),
    DisconnectMessage(Disconnect,SocketAddr),
    NeighborRequestMessage(NeighborRequest,SocketAddr),
    NeighborResponseMessage(NeighborResponse,SocketAddr),
    ShuffleMessage(Shuffle<'a>,SocketAddr),
    ShuffleReplyMessage(ShuffleReply<'a>,SocketAddr),
}

/// top-level function for serializing a HyParView message.
// TODO: adjust SocketAddr ro be a reference, rather than a copy (need to understand lifetimes for the HyParViewMessage struct)
pub fn deserialize(reader: &mut TcpStream, addr: SocketAddr) -> IoResult<HyParViewMessage> {
    match reader.read_u8() {
        Ok(0) => Ok(HyParViewMessage::JoinMessage(Join::new(), addr)),
        Ok(1) => Ok(HyParViewMessage::ForwardJoinMessage(ForwardJoin::deserialize(reader).ok().expect("failed to deserailize the forward join"), addr)),
        Ok(2) => Ok(HyParViewMessage::JoinAckMessage(JoinAck::new(), addr)),
        Ok(3) => Ok(HyParViewMessage::DisconnectMessage(Disconnect::new(), addr)),
        Ok(4) => Ok(HyParViewMessage::NeighborRequestMessage(NeighborRequest::deserialize(reader).ok().expect("failed to deserailize the neighbor request"), addr)),
        Ok(5) => Ok(HyParViewMessage::NeighborResponseMessage(NeighborResponse::deserialize(reader).ok().expect("failed to deserailize the neighbor response"), addr)),
        Ok(6) => Ok(HyParViewMessage::ShuffleMessage(Shuffle::deserialize(reader).ok().expect("failed to deserailize the shuffle"), addr)),
        Ok(7) => Ok(HyParViewMessage::ShuffleReplyMessage(ShuffleReply::deserialize(reader).ok().expect("failed to deserailize the shuffle reply"), addr)),
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

#[deriving(Copy,Show)]
pub struct NeighborRequest {
    pub priority: Priority,
}
impl NeighborRequest {
    pub fn new(priority: Priority) -> NeighborRequest {
        NeighborRequest { priority: priority}
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<NeighborRequest> {
        let p = match reader.read_u8().ok().expect("could not read priority from stream") {
            0 => Priority::Low,
            1 => Priority::High,
            //println!("received unknown priority level, defaulting to low priority");
            _ =>  Priority::Low,
        };
        Ok(NeighborRequest::new(p))
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_NEIGHBOR_REQUEST).ok();
        let p: u8 = match self.priority {
            Priority::Low => 0,
            Priority::High => 1,
        };
        writer.write_u8(p).ok();
        Ok(2)
    }
}

#[deriving(Copy,Show)]
pub struct NeighborResponse {
    pub result: Result,
}
impl NeighborResponse {
    pub fn new(result: Result) -> NeighborResponse {
        NeighborResponse { result: result }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<NeighborResponse> {
        let r = match reader.read_u8().ok().expect("could not read result from stream") {
            0 => Result::Accept,
            1 => Result::Reject,
            // if we can't undertand the result code, just default to reject
            _ => Result::Reject 
        };
        Ok(NeighborResponse::new(r))
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_NEIGHBOR_RESPONSE).ok();
        let p: u8 = match self.result {
            Result::Accept => 0,
            Result::Reject => 1,
        };
        writer.write_u8(p).ok();
        Ok(2)
    }
}

#[deriving(Show)]
pub struct Shuffle<'a> {
    pub originator: SocketAddr,
    pub nodes: &'a Vec<SocketAddr>,
    pub ttl: u8,
}
impl Shuffle<'a> {
    pub fn new(originator: SocketAddr, nodes: &Vec<SocketAddr>, ttl: u8) -> Shuffle {
        Shuffle { originator: originator, nodes: nodes, ttl: ttl }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<Shuffle> {
        let originator = deserialize_socket_addr(reader).ok().expect("could not read socket addr from stream");
        let nodes = deserialize_socket_addrs(reader);
        let ttl = reader.read_u8().ok().expect("could not read ttl from stream"); 
        Ok(Shuffle::new(originator, nodes, ttl))
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_SHUFFLE).ok();
        let mut cnt = 1;

        match serialize_socket_addr(&self.originator, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }

        match serialize_socket_addrs(&self.nodes, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }

        writer.write_u8(self.ttl).ok();
        cnt += 1;
        Ok(cnt)
    }
}

fn deserialize_socket_addrs(reader: &mut Reader) -> Vec<SocketAddr> {
    let len = reader.read_u8().ok().expect("could not read vector size from stream").to_uint().unwrap();
    let mut nodes = Vec::with_capacity(len);

    for i in range (0u, len) {
        match deserialize_socket_addr(reader) {
            Ok(socket) => nodes.push(socket),
            Err(e) => println!("failed"),
        }
    }
    nodes
}

fn serialize_socket_addrs(nodes: &Vec<SocketAddr>, writer: &mut Writer) -> IoResult<int> {
    let mut cnt = 0;
    writer.write_u8(nodes.len().to_u8().unwrap()).ok();
    cnt += 1;
    
    for node in nodes.iter() {
        match serialize_socket_addr(node, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }
    }
    Ok(cnt)
}

#[deriving(Show)]
pub struct ShuffleReply<'a> {
    /// return the vector of nodes the originator first sent out; that way the originator does not need to keep track of that.
    pub sent_nodes: &'a Vec<SocketAddr>,
    pub nodes: &'a Vec<SocketAddr>,
}
impl ShuffleReply<'a> {
    pub fn new(sent_nodes: &Vec<SocketAddr>, nodes: &Vec<SocketAddr>) -> ShuffleReply {
        ShuffleReply { sent_nodes: sent_nodes, nodes: nodes }
    }

    pub fn deserialize(reader: &mut Reader) -> IoResult<ShuffleReply> {
        let sent_nodes = deserialize_socket_addrs(reader);
        let nodes = deserialize_socket_addrs(reader);
        Ok(ShuffleReply::new(sent_nodes, nodes))
    }

    pub fn serialize(&self, writer: &mut Writer) -> IoResult<int> {
        writer.write_u8(HPV_MSG_ID_SHUFFLE_REPLY).ok();
        let mut cnt = 1;

        match serialize_socket_addrs(&self.sent_nodes, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }

        match serialize_socket_addrs(&self.nodes, writer) {
            Ok(c) => cnt += c,
            Err(e) => return Err(e),
        }
        Ok(cnt)
    }
}
