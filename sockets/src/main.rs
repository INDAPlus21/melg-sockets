use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;

fn main() {
  // 127.0.0.1 is address, 7878 is port
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }

  drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0 as u8; 50];

  // Read all data
  match stream.read(&mut buffer) {
    Ok(size) => {
      println!("handled");
      println!("{}", str::from_utf8(&buffer[0..size]).unwrap());
      stream.write(&buffer[0..size]).unwrap();
    }
    Err(_) => {
      println!("Error");
      //stream.shutdown(Shutdown::Both).unwrap();
    }
  }
  {}

  //stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
