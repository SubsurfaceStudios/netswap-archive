use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::Mutex;

use lazy_static::lazy_static;

mod buffer;

use crate::buffer::{BufferEntry, Buffer, permissions};

const CLIENT_RECV_BUF_SIZE_BYTES : usize = 50;
const PORT : &'static str = "3333";

lazy_static! {
    static ref BUFFER : Mutex<Buffer> = Mutex::new(Buffer::new());
}

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{PORT}").as_str()).unwrap();

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

#[test]
fn echo_connection_test() {
    match TcpStream::connect(format!("localhost:{PORT}")) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", stream.peer_addr().unwrap());

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent hello, awaiting reply...");

            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("ECHO test success.");
                    } else {
                        let text : &str = std::str::from_utf8(&data).unwrap();
                        panic!("ECHO test failed: {}", text);
                    }
                }, 
                Err(e) => panic!("Reception test failed: {}", e)
            }
        },
        Err(e) => panic!("Connection error: {}", e)
    }
}

fn connection(mut stream : TcpStream) {
    let mut recv_buffer = [0 as u8; CLIENT_RECV_BUF_SIZE_BYTES];

    while match stream.read(&mut recv_buffer) {
        Ok(size) => {
            // ECHO
            stream.write(&recv_buffer[0..size]).unwrap();
            true
        },
        Err(e) => {
            println!("Protocol error {}, terminating connection with {}.", e, stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}