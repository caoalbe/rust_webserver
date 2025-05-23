use std::net::{TcpListener, TcpStream};

pub mod common;
pub mod models;
pub use common::{HttpMethod, ThreadPool};
pub use models::{
    Request, Response,
    router::{CallbackHandler, Router},
};

pub struct Server {
    router: Router,
    thread_count: usize,
}

impl Server {
    pub fn build() -> &'static mut Server {
        let boxed = Box::new(Server {
            router: Router::new(),
            thread_count: 1,
        });
        Box::leak(boxed)
    }

    pub fn set_thread_count(&mut self, thread_count: usize) -> () {
        assert!(thread_count > 0);
        self.thread_count = thread_count;
    }

    pub fn listen(&'static self, port: u16, nullary_func: Option<fn()>) -> () {
        let address: String = format!("127.0.0.1:{}", port.to_string());
        let listener: TcpListener = TcpListener::bind(address).unwrap();

        let thread_pool: ThreadPool = ThreadPool::new(self.thread_count);

        if let Some(func) = nullary_func {
            func()
        }

        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();

            // Note: res must OWN stream; so req can only BORROW
            let req: Request = Request::new(&stream);
            let res: Response = Response::new(stream);

            let maybe_func: Option<&fn(Request, Response)> = self.router.query(req.get_key());
            match maybe_func {
                Some(func) => {
                    thread_pool.execute(move || {
                        func(req, res);
                    });
                }
                None => {}
            }
        }
    }

    // Maps callback to route with GET method
    pub fn get(&mut self, route: &str, callback: CallbackHandler) -> () {
        self.router.add_route(HttpMethod::GET, route, callback);
    }

    // Maps callback to route with PUT method
    pub fn put(&mut self, route: &str, callback: CallbackHandler) -> () {
        self.router.add_route(HttpMethod::PUT, route, callback);
    }

    // Maps callback to route with POST method
    pub fn post(&mut self, route: &str, callback: CallbackHandler) -> () {
        self.router.add_route(HttpMethod::POST, route, callback);
    }
}
