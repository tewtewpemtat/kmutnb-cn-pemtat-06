use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Running on port 8000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 14];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
        println!(
            "Got request from client:{:?}",
            std::str::from_utf8(&buffer).unwrap()
        );
    }
}
