use chrono::Local;
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
            println!("Successfully connected to chat. Write a message and press enter!");
            stream // Return unwraped stream
        }
        Err(_) => {
            println!("Error when connecting to stream");
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
                Ok(_) => {
                    print!(
                        "{}",
                        format!("YOU ({}): {}", Local::now().format("%H:%M"), input)
                    )
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
