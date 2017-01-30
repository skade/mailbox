use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7200").unwrap();

    // accept connections and process them, one by one
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let name = read_name(&mut s);
                write_greeting(&mut s, name);
            }
            Err(e) => {
                println!("A connection failed. Error: {}", e);
            }
        }
    }
}

fn read_name(stream: &mut TcpStream) -> String {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    let res = buffered_stream.read_line(&mut read_buffer);
    res.ok().expect("An error occured while reading!");
    read_buffer
}

fn write_greeting(stream: &mut TcpStream, name: String) {
    write!(stream, "Hello, {}!", name.trim());
}
