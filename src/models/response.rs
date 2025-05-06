use std::{io::Write, net::TcpStream};

pub struct Response {
    stream: TcpStream,
    http_version: String,
    http_status: String,
    contents: String,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response {
            stream,
            http_version: "HTTP/1.1".to_string(),
            http_status: "200 OK".to_string(),
            contents: "Hello Client!".to_string(),
        }
    }

    pub fn set_content(&mut self, new_content: String) -> () {
        self.contents = new_content;
    }

    pub fn send(&mut self) -> () {
        let output: String = format!(
            "{} {}\r\n\r\n{}\r\n",
            self.http_version, self.http_status, self.contents
        );
        self.stream.write_all(output.as_bytes()).unwrap();
    }
}
