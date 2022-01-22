use std::env;
use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpStream;
use std::str;

fn main() {
    // 127.0.0.1 is address, 7878 is port
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    loop {
        println!("Enter a message:");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match stream.write(input.as_bytes()) {
            Ok(_) => {
                let mut buffer = [0; 50];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("Returned: {}", str::from_utf8(&buffer[0..size]).unwrap());
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {
                // Exit if server has closed
                break;
            }
        }
    }
}
