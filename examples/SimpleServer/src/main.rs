use konnnyaku_server::server::server::Server;

fn main() {
    let mut server = Server::build(3000);
    server.get(String::from("/ping"), |_, res| {
        res.write("pong!");
    });
    server.serve();
}
