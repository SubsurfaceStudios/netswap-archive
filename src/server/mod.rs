use std::collections::HashMap;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::server::buffer::{BufferEntry, Buffer, permissions};
use crate::server::packet::{InboundPacket, OutboundPacket, Recievable, Sendable};

mod buffer;
mod ack;
mod packet;

const CLIENT_RECV_BUF_SIZE_BYTES : usize = 1500;
const MINIMUM_POSSIBLE_PACKET_SIZE_BYTES : usize = 10;

lazy_static! {
    static ref BUFFER : Mutex<Buffer> = Mutex::new(Buffer::new());
    static ref PACKET_BUFFER : Mutex<PacketBuffer<InboundPacket>> = Mutex::new(HashMap::new());
}

type PacketBuffer<T> = HashMap<u64, Vec<T>>;

fn init_server(port : &str) {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}").as_str()).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection with {}", stream.peer_addr().unwrap());
                thread::spawn(move || connection(stream));
            },
            Err(e) => eprintln!("Connection error {e}")
        }
    }

    drop(listener);
}

fn connection(mut stream : TcpStream) {
    let mut recv_buffer = [0 as u8; 1500];

    'read_loop: while match stream.read(&mut recv_buffer) {
        Ok(size) => {
            if size < MINIMUM_POSSIBLE_PACKET_SIZE_BYTES {
                malformed_packet(&stream);
                continue 'read_loop;
            }

            let xid = &recv_buffer[0..=3];
            let pidx = recv_buffer[4];
            let plen = recv_buffer[5];
            let headers = &recv_buffer[6..=10];
            let opcode = headers[0];

            let data = &recv_buffer[11..size];

            let packet = InboundPacket::construct(recv_buffer[0..size].to_vec());
            
            match packet {
                Some(p) => handle_packet(&stream, p),
                None => malformed_packet(&stream)
            };


            true
        },
        Err(e) => {
            println!("Protocol error {}, terminating connection with {}.", e, stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn malformed_packet(mut stream : &TcpStream) {
    stream.write(
        OutboundPacket {
            xid : 0,
            pid: 0,
            len: 0,
            headers : [0,0,0,0],
            opcode : ack::MALFORMED_PACKET_ERR_ACK,
            data : Vec::<u8>::new()
        }
        .send()
        .as_slice()
    );
}

fn handle_packet(stream : &TcpStream, packet : InboundPacket) {
    
}