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

    pub fn get(url: String) {
        let url = Url::new(&url);
        let request = Request::build(url.pathname, RequestMethod::GET, url.host);
        let host = request.host.clone();
        let request = request.print();
        let mut stream = Client::connect(host);
        let request = request.as_bytes();
        stream.write(request);
        stream.write(&[0]);
        let response = Response::parse_stream_to_response(&mut stream);
        println!("{}", response.print());
    }

    fn connect(host: String) -> TcpStream {
        let stream = TcpStream::connect(format!("{}:80", host));
        let stream = stream.unwrap();
        return stream;
    }
}
