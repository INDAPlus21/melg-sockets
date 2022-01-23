use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time;

fn main() {
  // 127.0.0.1 is address, 7878 is port
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let (send, recieve) = mpsc::channel();
  let (send_index, recieve_index) = mpsc::channel();
  let mut streams = Vec::new();
  let mut stream_index = 100;

  listener.set_nonblocking(true).unwrap(); // Prevent listener.accept blocking the thread

  // Main loop
  loop {
    // Listen for clients
    match listener.accept() {
      Ok((mut stream, _)) => {
        streams.push(stream.try_clone().unwrap());
        if stream_index == 100 {
          stream_index = 0;
        } else {
          stream_index += 1;
        }

        let local_sender = send.clone();
        let local_index_sender = send_index.clone();

        thread::spawn(move || {
          handle_connection(stream, local_sender, local_index_sender, stream_index);
        });
      }
      Err(_) => {}
    }

    // Go through all threads and send them the message that the server recieved
    loop {
      match recieve.try_recv() {
        Ok(message) => {
          if message.len() > 0 {
            println!("{}", message);
            let source_stream_index = recieve_index.recv().unwrap();
            println!("{:?}", source_stream_index);
            for i in 0..(&streams).len() {
              if i != source_stream_index {
                streams[i].write(message.as_bytes()).unwrap();
              }
            }
          }
        }
        Err(_) => break,
      }

      thread::sleep(time::Duration::from_secs(1));
    }
  }
}

fn handle_connection(
  mut stream: TcpStream,
  send: Sender<String>,
  send_index: Sender<usize>,
  stream_index: usize,
) {
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
        let message = str::from_utf8(&buffer[0..size]).unwrap();
        println!(
          "Message has been recieved on server: {} by {}",
          message, stream_index
        );
        send.send(message.to_owned()).unwrap();
        send_index.send(stream_index).unwrap();
      }
      Err(e) => {
        // Don't close connection
        if e.kind() == ErrorKind::WouldBlock {
          continue;
        }

        println!("{:?}", e);
        println!("Connection has been closed");
        stream.shutdown(Shutdown::Both).unwrap();
        break;
      }
    }

    thread::sleep(time::Duration::from_secs(1));
  }
}
