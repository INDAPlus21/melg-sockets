use chrono::Local;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::exit;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time;

fn main() {
  // 127.0.0.1 is address, 7878 is port
  let listener = match TcpListener::bind("127.0.0.1:7878") {
    Ok(stream) => {
      println!(
        "\x1b[32mSuccessfully connected server to chat. All messages sent will appear here\x1b[0m"
      );
      stream // Return unwraped stream
    }
    Err(_) => {
      println!("\x1b[31mError when connecting to stream\x1b[0m");
      exit(0); // Shut down program
    }
  };
  let (send, recieve) = mpsc::channel();
  let (send_index, recieve_index) = mpsc::channel();
  let mut streams = Vec::new();
  let mut stream_index = 100; // usize can't be -1 so use 100 instead

  listener.set_nonblocking(true).unwrap(); // Prevent listener.accept blocking the thread

  // Main loop
  loop {
    // Listen for clients
    match listener.accept() {
      Ok((stream, _)) => {
        println!("\x1b[32mSuccessfully connected a new client to the chat\x1b[0m");

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
            let source_stream_index = recieve_index.recv().unwrap();
            // Print to console (message already contains \n)
            print!(
              "{}",
              format!(
                "\x1b[34mCLIENT {} ({}):\x1b[0m {}",
                source_stream_index,
                Local::now().format("%H:%M"),
                format_message(&message)
              )
            );

            // Send to all clients except the source
            for i in 0..(&streams).len() {
              let mut prefix = String::from("\x1b[33mYOU");
              let mut private = false;

              if i != source_stream_index {
                prefix = format!("\x1b[34mCLIENT {}", source_stream_index);
              }

              // Private message
              if message.chars().nth(0).unwrap().is_digit(10) {
                let number = message.chars().nth(0).unwrap().to_digit(10).unwrap();

                if i != source_stream_index && i != (number as usize) {
                  continue; // Don't print
                } else {
                  prefix = format!("{} (PRIVATE)", prefix);
                  private = true;
                }
              }

              let mut formatted_message = format_message(&message);

              // Remove private message index
              if private {
                formatted_message = formatted_message[1..].to_owned();
              }

              match streams[i].write(
                format!(
                  "{} ({}):\x1b[0m {}",
                  prefix,
                  Local::now().format("%H:%M"),
                  formatted_message
                )
                .as_bytes(),
              ) {
                Ok(_) => {}
                Err(_) => {
                  // Don't crash when disconnecting stream. Don't remove from array as that messes up the thread indexes
                }
              }
            }
          }
        }
        // Let more clients connect after all messages have been said
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
  let mut buffer = [0 as u8; 1024];

  // Always listen for data
  loop {
    match stream.read(&mut buffer) {
      Ok(0) => {
        println!("\x1b[31mConnection to a client has been closed\x1b[0m");
        stream.shutdown(Shutdown::Both).unwrap();
        break;
      }
      Ok(size) => {
        let message = str::from_utf8(&buffer[0..size]).unwrap();
        // Send message and client index to main thread
        send.send(message.to_owned()).unwrap();
        send_index.send(stream_index).unwrap();
      }
      Err(e) => {
        // Don't close connection
        if e.kind() == ErrorKind::WouldBlock {
          continue;
        }

        println!("\x1b[31mConnection to a client has been closed\x1b[0m");
        stream.shutdown(Shutdown::Both).unwrap();
        break;
      }
    }
  }
}

static FORMATTING: [(&str, &str); 22] = [
  // Colours
  ("*bl", "\x1b[30m"),
  ("*r", "\x1b[31m"),
  ("*g", "\x1b[32m"),
  ("*y", "\x1b[33m"),
  ("*y", "\x1b[33m"),
  ("*b", "\x1b[34m"),
  ("*m", "\x1b[35m"),
  ("*c", "\x1b[36m"),
  ("*w", "\x1b[37m"),
  ("*0", "\x1b[0m"),
  // Emojis
  (":wave", "👋"),
  (":ok", "👌"),
  (":clap", "👏"),
  ("<3", "💓"),
  (":)", "🙂"),
  (":D", "😃"),
  (":c", "🙁"),
  (";c", "😢"),
  (":P", "😛"),
  (";P", "😜"),
  (":O", "😮"),
  (":/", "😕"),
];

// Replaces colours with actual colour commands and resets the colour afterwards
fn format_message(message: &String) -> String {
  let mut formatted_message = message.to_owned();
  for formatting_rule in &FORMATTING {
    formatted_message = formatted_message.replace(formatting_rule.0, formatting_rule.1);
  }

  formatted_message
}
