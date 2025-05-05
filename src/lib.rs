use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, str::FromStr,
};

pub struct Server {
    router: HashMap<(HttpMethod, String), fn()>,
}

#[derive(PartialEq, Eq, Hash)]
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

// TODO: Create a request, and response object

impl Server {
    pub fn build() -> Server {
        Server {
            router: HashMap::new(),
        }
    }

    // TODO: Add parameter for closure
    pub fn listen(&self, port: u16) -> () {
        let address: String = format!("127.0.0.1:{}", port.to_string());
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    pub fn get(&mut self, route: &str, callback: fn()) -> () {
        // bind route to func
        self.router
            .insert((HttpMethod::GET, route.to_string()), callback);
    }


    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
    
        // println!("Request: {http_request:#?}");


        let mut http_header = http_request[0].split_whitespace();
        let http_method: HttpMethod = http_header.next().unwrap_or("").parse().unwrap();
        let http_route = http_header.next().unwrap_or("");
        let http_version =http_header.next().unwrap_or("");

        let some_func = self.router.get_key_value(&(http_method, http_route.to_string()));
    
        match some_func {
            Some((_key, func)) => {
                func();
            },
            None => {}
        }

        // let status_line = "HTTP/1.1 200 OK";
        // let contents =
        //     "{\"message\": \"New user created\", \"user\": {\"firstName\": \"Paul\", \"Klee\"} }";
        // let response = format!("{status_line}\r\nContent-Type: application/json\r\n\r\n{contents}");
    
        // stream.write_all(response.as_bytes()).unwrap();
    }
    
}
