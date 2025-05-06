use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

pub struct Server {
    router: HashMap<(HttpMethod, String), CallbackHandler>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum HttpMethod {
    // TODO: This is not exhaustive.  See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
    GET,
    POST,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidHttpMethod;

impl FromStr for HttpMethod {
    type Err = InvalidHttpMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(InvalidHttpMethod),
        }
    }
}

pub struct Request {
    http_method: HttpMethod,
    http_route: String,
    http_version: String,
}

pub struct Response {
    status_line: String,
    contents: String,
}

// TODO: Should req and res be owned or borrowed in the callback?
//       If Response implements send(), I think it should be owned
//       We can expect the user to control when the server responds
type CallbackHandler = fn(req: &Request, res: &Response);

impl Response {
    fn new() -> Response {
        Response {
            status_line: "HTTP/1.1 200 OK".to_string(),
            contents: "Hello Client!".to_string(),
        }
    }

    fn generate_response(&self) -> String {
        let output: String = format!("{}\r\n\r\n{}\r\n", self.status_line, self.contents);
        output
    }
}

impl Request {
    fn new(stream: &TcpStream) -> Request {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // println!("Request: {http_request:#?}");

        let mut http_header = http_request[0].split_whitespace();

        let http_method: HttpMethod = http_header.next().unwrap_or("").parse().unwrap();
        let http_route = http_header.next().unwrap_or("").to_string();
        let http_version = http_header.next().unwrap_or("").to_string();
        Request {
            http_method,
            http_route,
            http_version,
        }
    }
}

impl Server {
    pub fn build() -> Server {
        Server {
            router: HashMap::new(),
        }
    }

    pub fn listen(&self, port: u16, nullary_func: Option<fn()>) -> () {
        let address: String = format!("127.0.0.1:{}", port.to_string());
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        if let Some(func) = nullary_func {
            func()
        }

        for stream in listener.incoming() {
            let mut stream: TcpStream = stream.unwrap();

            let req: Request = Request::new(&stream);
            let res: Response = Response::new();

            let maybe_func = self
                .router
                .get_key_value(&(req.http_method, req.http_route.clone()));

            match maybe_func {
                Some((_key, func)) => {
                    func(&req, &res);
                }
                None => {}
            }

            stream
                .write_all(&res.generate_response().as_bytes())
                .unwrap();
        }
    }

    pub fn get(&mut self, route: &str, callback: CallbackHandler) -> () {
        // bind route to func
        self.router
            .insert((HttpMethod::GET, route.to_string()), callback);
    }
}
