use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

pub mod common;
pub mod models;
pub use common::{HttpMethod, ThreadPool};
pub use models::{Request, Response};

pub struct Server {
    router: HashMap<(HttpMethod, String), CallbackHandler>,
}

type CallbackHandler = fn(req: Request, res: Response) -> ();

impl Server {
    pub fn build() -> &'static mut Server {
        let boxed = Box::new(Server {
            router: HashMap::new(),
        });
        Box::leak(boxed)
    }

    pub fn listen(&'static self, port: u16, nullary_func: Option<fn()>) -> () {
        let address: String = format!("127.0.0.1:{}", port.to_string());
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        let thread_pool: ThreadPool = ThreadPool::new(4);

        if let Some(func) = nullary_func {
            func()
        }

        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();

            // Note: res must OWN stream; so req can only BORROW
            let req: Request = Request::new(&stream);
            let res: Response = Response::new(stream);

            let maybe_func = self.router.get_key_value(&req.get_key());
            match maybe_func {
                Some((_key, func)) => {
                    thread_pool.execute(|| {
                        func(req, res);
                    });
                }
                None => {}
            }
        }
    }

    pub fn get(&mut self, route: &str, callback: CallbackHandler) -> () {
        // bind route to func
        self.router
            .insert((HttpMethod::GET, route.to_string()), callback);
    }
}
