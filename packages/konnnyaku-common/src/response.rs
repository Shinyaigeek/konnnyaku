use crate::stream::ApplicationStream;
use std::str;

pub struct Response {
    pub version: HTTPVersion,
    pub status_code: u32,
    pub body: Vec<u8>,
}

impl Response {
    pub fn build() -> Self {
        return Self {
            version: HTTPVersion::HttpOnePointOne,
            status_code: 200,
            body: vec![],
        };
    }

    pub fn parse_stream_to_response(stream: &mut ApplicationStream) -> Self {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        let mut idx = 0;

        let mut version = String::from("");
        while *&buffer[idx] != b' ' {
            version.push(*&buffer[idx] as char);
            idx += 1;
        }

        idx += 1;

        let version = match &version[..] {
            "HTTP/1.1" => HTTPVersion::HttpOnePointOne,
            "HTTP/1.0" => HTTPVersion::HttpOnePointZero,
            _ => panic!(""),
        };

        let mut status_code = String::from("");
        while *&buffer[idx] != b' ' {
            status_code.push(*&buffer[idx] as char);
            idx += 1;
        }

        idx += 1;
        let status_code = match &status_code[..] {
            "200" => 200,
            "404" => 404,
            _ => panic!("status code got {:?}", status_code),
        };

        let mut status = String::from("");
        while *&buffer[idx] != b'\n' {
            status.push(*&buffer[idx] as char);
            idx += 1;
        }

        idx += 1;

        while *&buffer[idx] != b'\r'
            || *&buffer[idx + 1] != b'\n'
            || *&buffer[idx + 2] != b'\r'
            || *&buffer[idx + 3] != b'\n'
        {
            idx += 1;
        }

        idx += 4;

        return Response {
            version,
            status_code,
            body: buffer[idx..].to_vec(),
        };
    }

    pub fn print(&self) -> String {
        let body = bytes_to_str(self.body.clone());
        String::from(format!(
            "HTTP/1.1 200 OK
Date: Sun, 10 Oct 2010 23:26:07 GMT
Server: Apache/2.2.8 (Ubuntu) mod_ssl/2.2.8 OpenSSL/0.9.8g
Last-Modified: Sun, 26 Sep 2010 22:04:35 GMT
ETag: \"45b6-834-49130cc1182c0\"
Accept-Ranges: bytes
Content-Length: {}
Connection: close
Content-Type: text/html

{}
",
            body.len(),
            body
        ))
    }

    pub fn write(&mut self, body: &str) {
        self.body.append(&mut body.to_string().as_bytes().to_vec());
    }
}

fn bytes_to_str(bytes: Vec<u8>) -> String {
    str::from_utf8(&bytes).unwrap().to_string()
}

pub enum HTTPVersion {
    HttpOnePointOne,
    HttpOnePointZero,
}
