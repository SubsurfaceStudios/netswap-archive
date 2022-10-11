pub struct InboundPacket {

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

        compile_error!("Finish implementing Sendable.");

        v        
    }
}

trait Sendable {
    fn send(&self) -> Vec<u8>;
}