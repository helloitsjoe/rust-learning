use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Server {}

impl Server {
  pub fn new() -> Server {
    println!("Creating server");
    Server {}
  }

  pub fn listen(&self, port: u32) {
    let listener = TcpListener::bind(String::from("localhost:") + &port.to_string()).unwrap();
    println!("listening on port {}", port);

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      handle_connection(stream);
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let contents = fs::read_to_string("resources/hello.html").unwrap();

  let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
