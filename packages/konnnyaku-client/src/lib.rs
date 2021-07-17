use crate::url::Url;
use konnnyaku_common::request::{Request, RequestMethod};
use konnnyaku_common::response::Response;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

mod url;

pub struct Client {}

impl Client {
    pub fn build() -> Self {
        return Self {};
    }

    pub fn get(url: String) -> Response {
        let url = Url::new(&Self::validate_url(url));
        let request = Request::build(url.pathname, RequestMethod::GET, url.host);
        let host = request.host.clone();
        let request = request.print();
        let mut stream = Client::connect(host);
        let request = request.as_bytes();
        stream.write(request);
        stream.write(&[0]);
        let response = Response::parse_stream_to_response(&mut stream);
        response
    }

    fn connect(host: String) -> TcpStream {
        // tls
        let stream = TcpStream::connect(format!("{}:80", host));
        let stream = stream.unwrap();
        return stream;
    }

    fn validate_url(url: String) -> String {
        if (url.ends_with("/")) {
            url
        } else {
            let mut url = url.clone();
            url.push('/');
            url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let response = Client::get("http://example.com/".to_string());
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn it_works_with_url_not_ending_with_slash() {
        let response = Client::get("http://example.com".to_string());
        assert_eq!(response.status_code, 200);
    }
}
