use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let start: Vec<&str> = req_str.split('\n').collect();

            let response: &[u8];

            if start[0].contains("/health") {
                response =
                    b"HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\nOK\r\n";
            } else {
                response = b"HTTP/1.1 200 OK\r\n";
            }
            match stream.write(response) {
                Ok(_) => println!("Response sent"),
                Err(e) => println!("Failed sending response: {}", e),
            }
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Listening for connections on port {}", 8000);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_read(&stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
