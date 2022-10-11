use crate::server::MINIMUM_POSSIBLE_PACKET_SIZE_BYTES;

pub struct InboundPacket {
    pub xid : u64,
    pub pid : u8,
    pub len : u8,
    pub opcode : u8,
    pub headers : [u8; 4],
    pub data : Vec<u8>
}

pub struct OutboundPacket {
    pub xid : u64,
    pub pid : u8,
    pub len : u8,
    pub opcode : u8,
    pub headers : [u8; 4],
    pub data : Vec<u8>
}

impl Sendable for OutboundPacket {
    fn send(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(10);

        let xid_bytes = bytemuck::bytes_of(&self.xid);

        for i in 0..4 {
            v.push(xid_bytes[i]);
        }

        v.push(self.pid);
        v.push(self.len);
        v.push(self.opcode);
        
        for i in 0..4 {
            v.push(self.headers[i]);
        }

        [v, self.data].concat()        
    }
}

impl Recievable for InboundPacket {
    fn construct(data : Vec<u8>) -> Option<Self> {
        if data.len() < MINIMUM_POSSIBLE_PACKET_SIZE_BYTES {
            return Option::None;
        }

        let xid : u64 = match bytemuck::try_from_bytes::<u64>(&data[0..4]) {
            Ok(val) => *val,
            Err(err) => {
                eprintln!("Failed to parse XID of packet, {}", err);
                return Option::None;
            }
        };

        let pid = data[4];
        let len = data[5];
        let opcode = data[6];

        let mut headers = [0 as u8; 4];

        for i in 7..=10 {
            headers[i-7] = data[i];
        }

        let packet_data = data[11..data.len()].to_vec();

        Some(
            Self {
                xid,
                pid,
                len,
                opcode,
                headers,
                data : packet_data
            }
        )
    }
}

pub trait Sendable {
    fn send(&self) -> Vec<u8>;
}

pub trait Recievable {
    fn construct(data : Vec<u8>) -> Option<Self> where Self : Sized;
}