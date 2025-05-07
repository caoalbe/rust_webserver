use crate::common::HttpMethod;
use std::{
    collections::HashMap,
    io::{BufReader, prelude::*},
    net::TcpStream,
};

pub struct Request {
    method: HttpMethod,
    full_route: String,
    path: String,
    query: HashMap<String, String>,
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

        // TODO: handle these Option instead of using unwrap_or
        let mut http_header = http_request[0].split_whitespace();

        let method: HttpMethod = http_header.next().unwrap_or("").parse().unwrap();

        let full_route = http_header.next().unwrap_or("").to_string();
        let mut route_iterator = full_route.split('?');
        let path = route_iterator.next().unwrap_or("").to_string();
        let query = route_iterator.next().unwrap_or("").to_string();
        let http_version = http_header.next().unwrap_or("").to_string();
        Request {
            method,
            full_route,
            path,
            query: Self::parse_query(query),
            http_version,
        }
    }

    fn parse_query(query_string: String) -> HashMap<String, String> {
        let mut output = HashMap::new();

        for pair in query_string.split('&') {
            // if let (str1, str2) = pair.split('=').collect()
            let mut pair = pair.split('=');
            let str1 = pair.next().unwrap_or("").to_string();
            let str2 = pair.next().unwrap_or("").to_string();
            output.insert(str1, str2);
        }

        output
    }

    pub fn get_full_route(&self) -> String {
        self.full_route.clone()
    }

    pub fn get_http_version(&self) -> String {
        self.http_version.clone()
    }

    pub fn get_query(&self, key: &str) -> String {
        self.query.get(key).unwrap_or(&"".to_string()).clone()
    }

    pub fn get_key(&self) -> (HttpMethod, String) {
        (self.method, self.path.clone())
    }
}
