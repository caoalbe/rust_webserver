use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub mod common;
pub mod models;
pub use common::HttpMethod;
pub use models::{Request, Response};

pub struct Server {
    router: HashMap<(HttpMethod, String), CallbackHandler>,
}

// TODO: Should req and res be owned or borrowed in the callback?
//       If Response implements send(), I think it should be owned
//       We can expect the user to control when the server responds
type CallbackHandler = fn(req: &Request, res: &Response);

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

            let maybe_func = self.router.get_key_value(&req.get_key());

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
