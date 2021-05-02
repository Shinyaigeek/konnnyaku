use konnnyaku_common::request::Request;
use konnnyaku_common::response::Response;
use std::io::Write;
use std::net::TcpListener;

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
            let request = Request::parse_stream_to_request(&mut stream);
            request.print();
            let response = Response::build().print();
            let response = response.as_bytes();
            stream.write(response);
            stream.flush();
        }
    }
}
