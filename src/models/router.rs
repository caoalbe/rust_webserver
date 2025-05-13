use crate::common::HttpMethod;
use crate::models::{Request, Response};
use std::collections::HashMap;

pub type CallbackHandler = fn(req: Request, res: Response) -> ();

pub struct Router {
    table: HashMap<(HttpMethod, String), CallbackHandler>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            table: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, method: HttpMethod, route: &str, callback: CallbackHandler) -> () {
        self.table.insert((method, route.to_string()), callback);
    }

    pub fn query(&self, key: &(HttpMethod, String)) -> Option<&CallbackHandler> {
        self.table.get(&key)
    }
}
