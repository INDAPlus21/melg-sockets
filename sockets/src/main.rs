use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn main() {
  // 127.0.0.1 is address, 7878 is port
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    thread::spawn(|| {
      let stream = stream.unwrap();
      handle_connection(stream);
    });
  }

  drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0 as u8; 50];

  // Always listen for data
  loop {
    match stream.read(&mut buffer) {
      Ok(0) => {
        println!("Connection has been closed");
        stream.shutdown(Shutdown::Both).unwrap();
        break;
      }
      Ok(size) => {
        println!(
          "Message has been recieved on server: {}",
          str::from_utf8(&buffer[0..size]).unwrap()
        );
        stream.write(&buffer[0..size]).unwrap();
        stream.flush().unwrap();
      }
      Err(e) => {
        println!("{:?}", e);
        println!("Connection has been closed");
        stream.shutdown(Shutdown::Both).unwrap();
        break;
      }
    }
  }
}
