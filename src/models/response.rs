use std::{io::Write, net::TcpStream};

pub struct Response {
    stream: TcpStream,
    status_line: String,
    contents: String,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response {
            stream,
            status_line: "HTTP/1.1 200 OK".to_string(),
            contents: "Hello Client!".to_string(),
        }
    }

    pub fn send(&mut self) -> () {
        let output: String = format!("{}\r\n\r\n{}\r\n", self.status_line, self.contents);
        self.stream.write_all(output.as_bytes()).unwrap();
    }
}
