use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 127.0.0.1 is address, 7878 is port
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    stream.write(args[1].as_bytes()).unwrap();

    let mut buffer = [0; 50];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Returned: {}", str::from_utf8(&buffer[0..size]).unwrap());
        }
        Err(_) => {}
    }

    //stream.write(args[1].as_bytes()).unwrap();
}
