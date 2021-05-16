#  konnnyaku

no-dependency HTTP client & server made by @Shinyaigeek.

## packages

* konnnyaku-common: utils for konnnyaku
* konnnyaku-client: http client of konnnyaku
* konnnyaku-cli: cli to use konnnyaku-client
* konnnyaku-server: http server of konnnyaku

## How to use

```
git clone git@github.com:Shinyaigeek/konnnyaku.git
```

### konnnyaku-server

```rust
use konnnyaku_server::server::server::Server;

fn main() {
    let mut server = Server::build(3000);
    server.get(String::from("/ping"), |req, res| {
        res.write("pong!");
    });
    server.serve();
}
```

### konnnyaku-client

```rust
use konnnyaku_client::Client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //* url
    let input = &args[1];
    Client::get(input.to_string());
}

```

## RoadMap 🚗

- [ ] HTTP method other than GET
- [ ] HTTP status message other than 200
- [ ] HTTP 2
- [ ] HTTP 3
- [ ] TLS