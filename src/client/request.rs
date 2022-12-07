use std::{collections::HashMap, ops::Range, fmt::Display, io::Write};

use super::method::Method;

static BOUNDARY: &str = "X_HTTPCLIENT_BOUNDARY";

#[derive(Clone)]
pub struct Request {
    method: Method,
    headers: HashMap<String, String>,
    query_strings: String,
    range: Option<Range<usize>>,
    body: Option<Vec<u8>>
}

impl Request {

    pub fn new() -> Request {
        Request { method: Method::GET, headers: HashMap::new(), query_strings: String::new(), range: None, body: None }
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

    pub fn get_query_strings(&self) -> &String {
        &self.query_strings
    }

    pub fn get_body(&self) -> &Option<Vec<u8>> {
        &self.body
    }

    pub fn get_content_length(&self) -> usize {
        if let Some(body) = &self.body {
            body.len()
        }else {
            0
        }
    }

}

impl Request {

    pub fn add_query_string<T: Display>(mut self, key: &str, value: T) -> Request {
        let mut item = String::new();
        if !self.get_query_strings().is_empty() {
            item.push('&');
        }
        item.push_str(key);
        item.push('=');
        item.push_str(&value.to_string());

        self.query_strings.push_str(&item);

        self
    }

}

impl Request {

    pub fn form_data(mut self) -> Request {
        self.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string()
        );
        self
    }

    pub fn add_form_data<T: Display>(mut self, key: &str, value: T) -> Request {
        let mut item = String::new();
        item.push_str(key);
        item.push('=');
        item.push_str(&value.to_string());

        if let Some(mut body) = self.body {
            item.insert(0, '&');
            body.write_all(item.as_bytes()).unwrap();
            self.body = Some(body)
        }else {
            self.body = Some(item.as_bytes().to_vec())
        }

        self
    }

}

impl Request {

    pub fn multipart(mut self) -> Request {
        self.headers.insert(
            "Content-Type".to_string(),
            format!("multipart/form-data; boundary=\"{}\"", BOUNDARY)
        );
        self
    }

    /**
     * --X_HTTPCLIENT_BOUNDARY
     * Content-Disposition: form-data; name="name"
     * 
     * value
     * --X_HTTPCLIENT_BOUNDARY
     * * --X_HTTPCLIENT_BOUNDARY
     * Content-Disposition: form-data; name="name"
     * 
     * value
     * --X_HTTPCLIENT_BOUNDARY--
     */

    pub fn add_data<T: Display>(mut self, key: &str, value: T) -> Request {
        let mut item = String::new();
        item.push_str(&format!(
            "Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key
        ));
        item.push_str(&format!("{}\r\n", value.to_string()));
        item.push_str(&format!("--{}\r\n", BOUNDARY));

        if let Some(mut body) = self.body {
            body.write_all(item.as_bytes()).unwrap();
            self.body = Some(body);
        }else {
            item.insert_str(0, &format!("--{}\r\n", BOUNDARY));
            self.body = Some(item.as_bytes().to_vec());
        }

        self
    }

    pub fn add_file(mut self, key: &str, file_name: &str, bytes: Vec<u8>) -> Request {
        let item = format!("Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n\r\n", key, file_name);

        if let Some(mut body) = self.body {
            body.write_all(item.as_bytes()).unwrap();
            body.write_all(&bytes).unwrap();
            body.write_all(format!("\r\n--{}\r\n", BOUNDARY).as_bytes()).unwrap();
            self.body = Some(body);
        }else {
            let mut body = format!("--{}\r\n", BOUNDARY).as_bytes().to_vec();
            body.write_all(item.as_bytes()).unwrap();
            body.write_all(&bytes).unwrap();
            body.write_all(format!("\r\n--{}\r\n", BOUNDARY).as_bytes()).unwrap();
        }
        self
    }

}