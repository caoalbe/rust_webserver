use std::str::FromStr;

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
