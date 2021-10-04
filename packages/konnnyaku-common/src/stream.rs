use crate::tls::{TlsConnector, TlsStream};
use crate::url::{Protocol, Url};
use std::io::{Read, Write};
use std::net::TcpStream;

pub enum ApplicationStream {
    TlsStream(TlsStream<TcpStream>),
    TcpStream(TcpStream),
}

impl ApplicationStream {
    pub fn new(url: &Url, tls_connector: &TlsConnector) -> Self {
        // tls
        let stream = TcpStream::connect(Self::make_connection_port(url));

        if url.is_https() {
            let tls_stream = tls_connector
                .connection
                .connect(&Self::make_connection_port(url), stream.unwrap())
                .unwrap();
            ApplicationStream::TlsStream(tls_stream)
        } else {
            ApplicationStream::TcpStream(stream.unwrap())
        }
    }

    fn make_connection_port(url: &Url) -> String {
        let port = match url.protocol {
            Protocol::Http => "80",
            Protocol::Https => "443",
        };

        format!("{}:{}", url.host, port)
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        match self {
            ApplicationStream::TlsStream(stream) => stream.write(data),
            ApplicationStream::TcpStream(stream) => stream.write(data),
        }
    }

    pub fn read_to_end(&mut self, data: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        match self {
            ApplicationStream::TlsStream(stream) => stream.read_to_end(data),
            ApplicationStream::TcpStream(stream) => stream.read_to_end(data),
        }
    }
}
