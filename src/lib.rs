use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub struct Server {}

// TODO: Create a request, and response object

impl Server {
    // TODO: Add parameter for closure
    pub fn listen(&self, port: u16) -> () {
        let address: String = format!("127.0.0.1:{}", port.to_string());
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    }

    pub fn get(&self, route: &str, func: String) -> () {
        // bind route to func
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents =
        "{\"message\": \"New user created\", \"user\": {\"firstName\": \"Paul\", \"Klee\"} }";
    let response = format!("{status_line}\r\nContent-Type: application/json\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
