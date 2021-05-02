use konnnyaku_client::Client;

fn main() {
    let client = Client::build();
    Client::get(String::from("asdf"));
}
