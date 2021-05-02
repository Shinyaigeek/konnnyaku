use std::net::TcpListener;
use std::io::{Write, Read};
use std::str;

pub struct Server {
    port: i32,
    listener: TcpListener,
}

impl Server {
    pub fn build(port: i32) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).unwrap();
        Self { port, listener }
    }

    pub fn serve(self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection from {}", stream.peer_addr().unwrap());
            let mut buffer = [0; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap();
                println!("bytes:{:?}", buffer);
                let http = str::from_utf8(&buffer).unwrap();
                println!("http : {:?}", http);
                stream.write(&buffer[..nbytes]).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
