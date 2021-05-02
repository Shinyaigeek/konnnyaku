pub struct Response {
    version: HTTPVersion,
    status_code: u32,
    body: String,
}

impl Response {
    pub fn build() -> Self {
        return Self {
            version: HTTPVersion::one_one,
            status_code: 200,
            body: "".to_string(),
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
Content-Length: 4
Connection: close
Content-Type: text/html

{:?}",
            self.body
        ))
    }
}

const one_one: &str = "HTTP/1.1";

enum HTTPVersion {
    one_one,
}

fn match_status(status_code: u32) -> String {
    match status_code {
        200 => "OK".to_string(),
        _ => panic!(""),
    }
}
