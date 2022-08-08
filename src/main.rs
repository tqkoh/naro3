use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
  let address = match env::var("NARO3_ADDRESS"){
    Ok(val) => val,
    Err(_e) => "localhost:8080".to_string()
  };
  let listener = TcpListener::bind(address).unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let contents = "Hello, World!";

  let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
