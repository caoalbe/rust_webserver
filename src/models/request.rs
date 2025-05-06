use crate::common::HttpMethod;
use std::{
    io::{BufReader, prelude::*},
    net::TcpStream,
};

pub struct Request {
    http_method: HttpMethod,
    http_route: String,
    http_version: String,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Request {
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

    pub fn get_key(&self) -> (HttpMethod, String) {
        (self.http_method, self.http_route.clone())
    }
}
