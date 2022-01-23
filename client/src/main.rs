use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpStream;
use std::process::exit;
use std::str;
use std::thread;

fn main() {
    // 127.0.0.1 is address, 7878 is port
    let mut stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!(
                "\x1b[32mSuccessfully connected client to chat. Write a message and press enter!\x1b[0m"
            );
            stream // Return unwraped stream
        }
        Err(_) => {
            println!("\x1b[31mError when connecting to stream\x1b[0m");
            exit(0); // Shut down program
        }
    };

    // Send messages
    let mut stream_copy = stream.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match stream_copy.write(input.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    // Exit if server has closed
                    break;
                }
            }
        }
    });

    // Print messages from other clients
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => {}
            Ok(size) => {
                print!("{}", str::from_utf8(&buffer[0..size]).unwrap());
            }
            Err(_) => {}
        }
    }
}
