use std::{collections::HashMap, ops::Range};

use super::method::Method;


pub struct Request {
    method: Method,
    headers: HashMap<String, String>,
    range: Option<Range<usize>>
}

impl Request {

    pub fn new() -> Request {
        Request { method: Method::GET, headers: HashMap::new(), range: None }
    }

    pub fn set_method(mut self, method: Method) -> Request {
        self.method = method;
        self
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn set_headers(mut self, headers: HashMap<String, String>) -> Request {
        self.headers = headers;
        self
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn set_range(mut self, range: Range<usize>) -> Request {
        self.range = Some(range);
        self
    }

    pub fn get_range(&self) -> &Option<Range<usize>> {
        &self.range
    }

}