use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

struct RequestHeader {}

const GET: &str = "GET";
const POST: &str = "POST";
const DELETE: &str = "DELETE";
const PUT: &str = "PUT";
const PATCH: &str = "PATCH";
const OPTIONS: &str = "OPTIONS";
const HEAD: &str = "HEAD";
const CONNECT: &str = "CONNECT";
const TRACE: &str = "TRACE";

pub enum RequestMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

pub struct Request {
    method: RequestMethod,
    pub url: String,
    pub host: String,
    header: RequestHeader,
}

impl Request {
    pub fn parse_stream_to_request(stream: &mut TcpStream) -> Self {
        let mut buffer = [0; 2048];
        loop {
            let nbytes = stream.read(&mut buffer).unwrap();
            let http = str::from_utf8(&buffer).unwrap();
            let mut idx = 0;
            let method = parse_method(&mut idx, &buffer);
            let url = parse_url(&mut idx, &buffer);
            return Self {
                method,
                url,
                header: RequestHeader {},
                // TODO
                host: String::from(""),
            };
        }
    }

    pub fn build(url: String, method: RequestMethod, host: String) -> Self {
        Self {
            url,
            method,
            header: RequestHeader {},
            host,
        }
    }

    pub fn print(&self) -> String {
        let method = match self.method {
            RequestMethod::GET => GET,
            RequestMethod::POST => POST,
            RequestMethod::PUT => PUT,
            RequestMethod::PATCH => PATCH,
            RequestMethod::HEAD => HEAD,
            RequestMethod::DELETE => DELETE,
            RequestMethod::OPTIONS => OPTIONS,
            RequestMethod::CONNECT => CONNECT,
            RequestMethod::TRACE => TRACE,
        };
        let url = &self.url;
        String::from(format!(
            "{} {} HTTP/1.0\r
Host: {}\r
User-Agent: curl/7.54.0\r
Accept: */*\r
\r
",
            method, url, self.host
        ))
    }
}

fn parse_method(_idx: &mut usize, buffer: &[u8]) -> RequestMethod {
    let mut method: String = String::from("");
    let mut idx = _idx.clone();
    while buffer[idx] != b' ' {
        let char = buffer[idx] as char;
        method.push(char);
        idx += 1;
    }

    *_idx = idx + 1;

    let method: &str = &method;

    match method {
        GET => RequestMethod::GET,
        POST => RequestMethod::POST,
        HEAD => RequestMethod::HEAD,
        DELETE => RequestMethod::DELETE,
        PUT => RequestMethod::PUT,
        PATCH => RequestMethod::PATCH,
        OPTIONS => RequestMethod::OPTIONS,
        TRACE => RequestMethod::TRACE,
        CONNECT => RequestMethod::CONNECT,
        _ => panic!(
            "method should be GET or POST or HEAD or ...., but got {:?}",
            method
        ),
    }
}

fn parse_url(_idx: &mut usize, buffer: &[u8]) -> String {
    let mut url: String = String::from("");
    let mut idx = _idx.clone();
    while buffer[idx] != b' ' {
        let char = buffer[idx] as char;
        url.push(char);
        idx += 1;
    }

    let g = String::from("GET");
    *_idx = idx + 1;

    return url;
}
