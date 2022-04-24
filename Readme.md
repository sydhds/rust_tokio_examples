
## Howto

* `cd rust_01_hello_world`
* `cargo run`

## Examples

### Rust

* rust_01_...: rust basics - println!
* rust_02_...: rust basics - variables / tuples / parse int / array
* rust_03_...: rust basics - functions
* rust_04_...: rust basics - if / loop / while / for
* rust_05_...: rust basics - match
* rust_06_...: rust basics - ownership / borrowing (basic)
* rust_07_...: rust basics - vector / hash map
* rust_08_...: rust basics - struct / struct functions (e.g. ~ class methods)
* rust_09_...: rust basics - traits
* rust_10_...: rust basics - unit tests

### Tokio

* tokio_tcp_echo: an uppercase echo tcp server
  * run the server:
    * cargo run
  * for the client, use nc:
    * nc 127.0.0.1 6161 (Ctrl-C to exit)

### Hyper (http)

* hyper_01_http_post: http server & client using hyper crate
  * run the server:
    * cargo run
  * run the client (or use curl, cmd line example in src files)
    * cargo run --example 03_client

