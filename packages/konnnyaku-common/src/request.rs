use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

struct RequestHeader {}

const GET: &str = "GET";

enum RequestMethod {
    GET,
}

pub struct Request {
    method: RequestMethod,
    url: String,
    header: RequestHeader,
}

impl Request {
    pub fn parse_stream_to_request(stream: &mut TcpStream) {
        let mut buffer = [0; 1024];
        loop {
            let nbytes = stream.read(&mut buffer).unwrap();
            println!("bytes:{:?}", buffer);
            let http = str::from_utf8(&buffer).unwrap();
            println!("http : {:?}", http);
            println!("----------");
            let mut idx = 0;
            let method = parse_method(&mut idx, &buffer);
            match method {
                RequestMethod::GET => println!("GET")
            };
            return;
        }
    }

    pub fn print(&self) {
        // println!("HTTP Request Method: {}", self.method);
    } 
}

fn parse_method(_idx: &mut usize, buffer: &[u8]) -> RequestMethod {
    let mut method: String = String::from("");
    let mut idx = _idx.clone();
    while buffer[idx] != b' '{
        let char = buffer[idx] as char;
        method.push(char);
        idx += 1;
    }

    let g = String::from("GET");
    *_idx = idx;

    match method {
        g => RequestMethod::GET
    }
}
