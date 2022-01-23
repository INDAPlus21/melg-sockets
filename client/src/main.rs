use std::env;
use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time;

fn main() {
    // 127.0.0.1 is address, 7878 is port
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // Send messages
    let mut stream_copy = stream.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match stream_copy.write(input.as_bytes()) {
                Ok(_) => {
                    // Successfully sent message
                }
                Err(_) => {
                    // Exit if server has closed
                    break;
                }
            }
        }
    });

    // Print messages
    loop {
        let mut buffer = [0; 50];
        match stream.read(&mut buffer) {
            Ok(0) => {}
            Ok(size) => {
                print!("{}", str::from_utf8(&buffer[0..size]).unwrap());
            }
            Err(_) => {}
        }
    }
}
