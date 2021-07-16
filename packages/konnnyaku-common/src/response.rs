use std::io::Read;
use std::net::TcpStream;
use std::str;

pub struct Response {
    version: HTTPVersion,
    pub status_code: u32,
    pub body: String,
}

impl Response {
    pub fn build() -> Self {
        return Self {
            version: HTTPVersion::one_one,
            status_code: 200,
            body: "".to_string(),
        };
    }

    pub fn parse_stream_to_response(stream: &mut TcpStream) -> Self {
        let mut buffer = Vec::new();
        let nbytes = stream.read_to_end(&mut buffer).unwrap();

        let mut idx = 0;

        let mut version = String::from("");
        while *&buffer[idx] != b' ' {
            version.push(*&buffer[idx] as char);
            idx += 1;
        }

        idx += 1;

        let version = match &version[..] {
            one_one => HTTPVersion::one_one,
            one_zero => HTTPVersion::one_zero,
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

        let header = -1;

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
            body: str::from_utf8(&buffer[idx..]).unwrap().to_string(),
        };
    }

    pub fn print(&self) -> String {
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
            self.body.len(),
            self.body
        ))
    }

    pub fn write(&mut self, body: &str) {
        self.body.push_str(body);
    }
}

const one_one: &str = "HTTP/1.1";
const one_zero: &str = "HTTP/1.0";

enum HTTPVersion {
    one_one,
    one_zero,
}

fn match_status(status_code: u32) -> String {
    match status_code {
        200 => "OK".to_string(),
        404 => "Not Found".to_string(),
        _ => panic!(""),
    }
}
