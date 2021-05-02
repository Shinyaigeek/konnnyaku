use konnnyaku_client::Client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    Client::get(input.to_string());
}
