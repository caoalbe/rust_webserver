pub struct Response {
    status_line: String,
    contents: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            status_line: "HTTP/1.1 200 OK".to_string(),
            contents: "Hello Client!".to_string(),
        }
    }

    pub fn generate_response(&self) -> String {
        let output: String = format!("{}\r\n\r\n{}\r\n", self.status_line, self.contents);
        output
    }
}
