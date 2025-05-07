use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct Response {
    stream: TcpStream,
    http_version: String,
    http_status: String,
    headers: HashMap<String, String>,
    headers_sent: bool,
    contents: String,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response {
            stream,
            http_version: "HTTP/1.1".to_string(),
            http_status: "200 OK".to_string(),
            headers: HashMap::new(),
            headers_sent: false,
            contents: "".to_string(),
        }
    }

    pub fn append_header(&mut self, field: String, value: String) -> () {
        self.headers.insert(field, value);
    }

    pub fn set_content(&mut self, new_content: String) -> () {
        self.contents = new_content;
    }

    pub fn send(&mut self) -> () {
        let mut http_header: String = String::new();
        for (key, value) in self.headers.clone() {
            http_header.push_str(&format!("{}: {}\n", key, value).to_string());
        }

        let output: String = format!(
            "{} {}\r\n{}\r\n\r\n{}\r\n",
            self.http_version, self.http_status, http_header, self.contents
        );
        self.stream.write_all(output.as_bytes()).unwrap();
        self.headers_sent = true;
    }
}
