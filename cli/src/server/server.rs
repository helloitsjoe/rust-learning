use super::lib::ThreadPool;
use std::{
  fs,
  io::prelude::*,
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};

pub struct Server {}

impl Server {
  pub fn new() -> Server {
    println!("Creating server");
    Server {}
  }

  pub fn listen(&self, port: u32) {
    let listener = TcpListener::bind(String::from("localhost:") + &port.to_string()).unwrap();
    println!("listening on port {}", port);
    let mut pool = ThreadPool::new(2);

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      pool.execute(|| {
        handle_connection(stream);
      });
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "hello.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(10));
    ("HTTP/1.1 200 OK", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let path = format!("resources/{}", filename);
  let contents = fs::read_to_string(path).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
