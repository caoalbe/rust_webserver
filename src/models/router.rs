use crate::common::HttpMethod;
use crate::models::{Request, Response};
use regex::Regex;
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
        self.table.insert((method, path_to_regex(route)), callback);
    }

    pub fn query(
        &self,
        (target_method, target_route): (HttpMethod, String),
    ) -> Option<&CallbackHandler> {
        let mut re: Regex;
        for key in self.table.keys() {
            let (key_method, key_route) = key;
            re = Regex::new(key_route).unwrap();
            if &target_method == key_method && re.is_match(&target_route) {
                return self.table.get(key);
            }
        }
        return None;
    }
}

fn path_to_regex(mut path: &str) -> String {
    if path.starts_with('/') {
        path = &path[1..];
    }

    let mut regex = String::from("^");
    for segment in path.split('/') {
        if segment.starts_with(':') {
            let name = &segment[1..];
            regex.push_str(&format!("/(?P<{name}>[^/]+)"));
        } else {
            regex.push_str(&format!("/{segment}"));
        }
    }
    regex.push('$');
    regex
}

#[test]
fn test_path_to_regex() {
    assert_eq!(path_to_regex("/"), "^/$");
    assert_eq!(path_to_regex("//"), "^//$");
    assert_eq!(path_to_regex("/slow"), "^/slow$");
    assert_eq!(path_to_regex("/one/two"), "^/one/two$");
    assert_eq!(
        path_to_regex("/books/:booksId"),
        "^/books/(?P<booksId>[^/]+)$"
    );
    // Without leading slash
    assert_eq!(path_to_regex("slow"), "^/slow$");
    assert_eq!(path_to_regex("one/two"), "^/one/two$");
    assert_eq!(
        path_to_regex("books/:booksId"),
        "^/books/(?P<booksId>[^/]+)$"
    );
}
