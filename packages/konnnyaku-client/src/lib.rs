use konnnyaku_common::request::{Request, RequestMethod};
use konnnyaku_common::response::Response;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub struct Client {}

impl Client {
    pub fn build() -> Self {
        return Self {};
    }

    pub fn get(url: String) {
        let request = Request::build(String::from("/ping"), RequestMethod::GET);
        let request = request.print();
        let mut stream = Client::connect(String::from("127.0.0.1:3000"));
        stream.write(request.as_bytes());
        let mut buffer = [0; 2048];
        let nbytes = stream.read(&mut buffer).unwrap();
        let http = str::from_utf8(&buffer).unwrap();
        println!("http: {}", http);
    }

    fn connect(host: String) -> TcpStream {
        let stream = TcpStream::connect(host);
        let stream = stream.unwrap();
        return stream;
    }
}
