use bytemuck;

pub const GENERIC_OP_SUCCESS_ACK : u8 = 0;
pub const FAILURE_ACCESS_VIOLATION_ACK : u8 = 1;
pub const FAILURE_ADDRESS_FAULT_ACK : u8 = 2;
pub const RESOLVE_POINTER_OP_ACK : u8 = 3;

/// Any ACK which can be sent as a single packet with no data, guaranteed.
pub trait HeaderOnlyAck {
    /// Convert the packet into the proper bytes to be sent over TCP.
    fn as_bytes(&self) -> Box<[u8]>;
}

/// A fragment of an `impl DataPacketAck`.
pub trait DataPacketFragment {
    /// Convert the fragment into the proper bytes to be sent over TCP.
    fn as_bytes(&self) -> Vec<u8>;
}

/// Any ACK which has the possibility of requiring fragmentation. (eg. uses the Data block)
pub trait DataPacketAck {
    /// Convert this ACK into a `Vec` of the packets to be sent to the client.
    fn as_packet_vec(&self) -> Vec<Box<[u8]>>;
}

/// Generic response for any non-data related packet.
pub struct GenericOpSuccessAck {
    /// Transaction ID - XID
    pub xid : u64,
    /// Packet Index - PIDX
    pub pidx : u8
}

/// Generic response for Access Violation op failures.
pub struct FailureAccessViolationAck {
    /// Transaction ID - XID
    pub xid : u64,
    /// Packet Index - PIDX
    pub pidx : u8
}

/// Generic response for Address Fault op failures.
pub struct FailureAddressFaultAck {
    /// Transaction ID - XID
    pub xid : u64,
    /// Packet Index - PIDX
    pub pidx : u8
}

/// Pointer dereference operation ACK.
pub struct ResolvePointerOpAck {
    /// Transaction ID - XID
    pub xid : u64,
    /// Data array - not limited to 1 packet.
    pub data : Box<[u8]>
}

pub struct ResolvePointerOpAckFragment {
    pub xid : u64,
    pub pidx : u8,
    pub data : Box<[u8]>
}

impl DataPacketAck for ResolvePointerOpAck {
    fn as_packet_vec(&self) -> Vec<Vec<u8>> {
        if self.data.len() < 1390 {
            let p = ResolvePointerOpAckFragment {
                xid: todo!(),
                pidx: todo!(),
                data: todo!(),
            };

            return vec![p.as_bytes()];
        }

        todo!();
    }
}

impl DataPacketFragment for ResolvePointerOpAckFragment {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes : Vec<u8> = Vec::new(); // Pre allocate for headers.

        let xid_bytes = bytemuck::bytes_of(&self.xid);

        bytes.push(xid_bytes[0]);
        bytes.push(xid_bytes[1]);
        bytes.push(xid_bytes[2]);
        bytes.push(xid_bytes[3]);

        bytes.push(self.pidx);

        bytes
    }
}

impl HeaderOnlyAck for GenericOpSuccessAck {
    fn as_bytes(&self) -> Box<[u8]> {
        let mut data = [0 as u8; 10];

        let xid : &[u8] = bytemuck::bytes_of(&self.xid);

        data[0] = xid[0];
        data[1] = xid[1];
        data[2] = xid[2];
        data[3] = xid[3];

        data[4] = self.pidx;

        // Packet length - this is a single packet response.
        data[5] = 1;

        // Headers 0-4 - we only need to set Header 1 (OPCODE) because all the others are initialized to 0000 0000.
        data[6] = GENERIC_OP_SUCCESS_ACK;

        Box::new(data)
    }
}

impl HeaderOnlyAck for FailureAccessViolationAck {
    fn as_bytes(&self) -> Box<[u8]> {
        let mut data = [0 as u8; 10];

        let xid : &[u8] = bytemuck::bytes_of(&self.xid);

        data[0] = xid[0];
        data[1] = xid[1];
        data[2] = xid[2];
        data[3] = xid[3];

        data[4] = self.pidx;

        // Packet length - this is a single packet response.
        data[5] = 1;

        // Headers 0-4 - we only need to set Header 1 (OPCODE) because all the others are initialized to 0000 0000.
        data[6] = GENERIC_OP_SUCCESS_ACK;

        Box::new(data)
    }
}

impl HeaderOnlyAck for FailureAddressFaultAck {
    fn as_bytes(&self) -> Box<[u8]> {
        let mut data = [0 as u8; 10];

        let xid : &[u8] = bytemuck::bytes_of(&self.xid);

        data[0] = xid[0];
        data[1] = xid[1];
        data[2] = xid[2];
        data[3] = xid[3];

        data[4] = self.pidx;

        // Packet length - this is a single packet response.
        data[5] = 1;

        // Headers 0-4 - we only need to set Header 1 (OPCODE) because all the others are initialized to 0000 0000.
        data[6] = GENERIC_OP_SUCCESS_ACK;

        Box::new(data)
    }
}