use std::fmt;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum HttpStatus {
    // TODO: This is not exhaustive: See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
    OK = 200,
}

impl HttpStatus {
    fn get_pair(status: &HttpStatus) -> (u16, String) {
        match status {
            HttpStatus::OK => (200, "OK".to_string()),
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (num, text) = HttpStatus::get_pair(self);
        write!(f, "{num} {text}")
    }
}
