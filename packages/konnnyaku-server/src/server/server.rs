use konnnyaku_common::request::Request;
use konnnyaku_common::response::Response;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;

pub struct Server {
    port: i32,
    listener: TcpListener,
    handlers: HashMap<String, fn(Request, &mut Response)>,
}

impl Server {
    pub fn build(port: i32) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).unwrap();
        let handlers = HashMap::<String, fn(Request, &mut Response)>::new();
        Self {
            port,
            listener,
            handlers,
        }
    }

    pub fn get(&mut self, url: String, handler: fn(Request, &mut Response)) {
        self.handlers.insert(url, handler);
    }

    pub fn serve(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection from {}", stream.peer_addr().unwrap());
            let request = Request::parse_stream_to_request(&mut stream);
            request.print();
            let mut response = Response::build();
            let callback_handler = self.handlers.get(&request.url).unwrap();
            callback_handler(request, &mut response);
            let response = response.print();
            let response = response.as_bytes();
            stream.write(response);
            stream.flush();
        }
    }
}
