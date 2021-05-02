use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

struct RequestHeader {}

const GET: &str = "GET";

pub enum RequestMethod {
    GET,
}

pub struct Request {
    method: RequestMethod,
    pub url: String,
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
            match method {
                RequestMethod::GET => println!("GET"),
            };
            return Self {
                method,
                url,
                header: RequestHeader {},
            };
        }
    }

    pub fn build(url: String, method: RequestMethod) -> Self {
        Self {
            url,
            method,
            header: RequestHeader {},
        }
    }

    pub fn print(&self) -> String {
        let method = match self.method {
            RequestMethod::GET => "GET",
        };
        let url = &self.url;
        String::from(format!(
            "{} {} HTTP/1.1
User-Agent: konnnyaku
Accept: *",
            method, url
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

    let g = String::from("GET");
    *_idx = idx + 1;

    match method {
        g => RequestMethod::GET,
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
