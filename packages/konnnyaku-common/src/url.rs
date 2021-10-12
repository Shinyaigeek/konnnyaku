pub struct Url {
    pub host: String,
    pub href: String,
    pub origin: String,
    pub protocol: Protocol,
    pub pathname: String,
}

impl Url {
    pub fn new(href: &str) -> Self {
        let mut idx = 0;
        let mut protocol = String::from("");
        let href_bytes = href.as_bytes();
        while href_bytes[idx] != b':' {
            protocol.push(href_bytes[idx] as char);
            idx += 1;
        }

        protocol.push(':');
        idx += 3;

        let protocol = match &protocol[..] {
            "http:" => Protocol::Http,
            "https:" => Protocol::Https,
            _ => panic!("{}", protocol),
        };

        let mut host = String::from("");

        while href_bytes.len() > idx
            && href_bytes[idx] != b'/'
            && href_bytes[idx] != b'?'
            && href_bytes[idx] != b'#'
        {
            host.push(href_bytes[idx] as char);
            idx += 1;
        }

        let mut pathname = String::from("");

        while href.len() > idx {
            pathname.push(href_bytes[idx] as char);
            idx += 1;
        }

        let pathname = if pathname.len() == 0 {
            "/".to_string()
        } else {
            pathname
        };

        let origin = format!("{}//{}", "http:", host);

        return Self {
            host,
            href: href.to_string(),
            origin,
            protocol,
            pathname,
        };
    }

    pub fn is_https(&self) -> bool {
        self.protocol == Protocol::Https
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Protocol {
    Http,
    Https,
}
