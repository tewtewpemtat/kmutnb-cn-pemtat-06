use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:8000").unwrap();
    stream.write("Hello Myfriend".as_bytes()).unwrap();
    let mut buffer = [0; 14];
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
