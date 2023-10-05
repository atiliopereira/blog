use std::fs;

use crate::url::{get_status_code_str, StatusCode};

pub struct Response {
    pub body: String,
    pub headers: Vec<String>,
    pub http_version: String,
    pub status_code: StatusCode,
}

impl Response {
    pub fn new(status_code: StatusCode, path: &str) -> Self {
        let http_version = String::from("HTTP/1.1");
        let mut headers = Vec::<String>::new();
        let content_type = String::from("Content-Type: text/html; charset=UTF-8");
        let server = String::from("Server: chilu's blog server");
        headers.push(content_type);
        headers.push(server);
        let body = fs::read_to_string(path).unwrap();

        Self { body, headers, http_version, status_code,}
    }

    pub fn to_sting(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.http_version);
        output.push_str(" ");
        output.push_str(get_status_code_str(&self.status_code));
        output.push_str("\r\n");

        for header in &self.headers {
            output.push_str(&header);
            output.push_str("\r\n");
        }

        output.push_str("\r\n");
        output.push_str(&self.body);

        output
    }
}