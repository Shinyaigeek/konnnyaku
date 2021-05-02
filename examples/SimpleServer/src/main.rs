use konnnyaku_server::server::server::Server;

fn main() {
    let server = Server::build(3000);
    println!("Hello, world!");
}
