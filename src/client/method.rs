use std::fmt::Display;

#[derive(Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    HEAD,
    OPTIONS,
    DELETE
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::PATCH => write!(f, "PATCH"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::DELETE => write!(f, "DELETE"),
        }
    }
}