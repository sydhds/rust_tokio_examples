
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
* rust_11_...: rust basics - traits 2 aka Box aka function return different Struct (impl trait Describe)
* rust_12_...: rust basics - closures aka lambda
* rust_13_...: rust basics - spawn thread(s) + map reduce example
* rust_14_...: rust basics - channel to communicate between threads (mpsc: multi producer, single consumer)
* rust_15_...: implementing an even number iterator + defaults for function arg
* rust_16_...: another way to implement defaults for function arg (impl trait From / Default)
* rust_17_...: rust basics - error handling + keyword: ?
* rust_18_...: sharing a value between threads with Atomic Reference Counting (Arc)
* rust_19_...: function that can return multi error types (complex - 4 != implementations)
* rust_20_...: thread + Mutex (+ Arc)
* rust_21_...: rust basics - enum (enum with data)
* rust_22_...: impl Into for a struct
* rust_23_...: generic function / struct
* rust_24_...: implicit type conversion (coercion)
* rust_25_...: Path & PathBuf (filepath) + generic: (AsRef<Path>) / (Into<PathBuf>)

### Tokio

* tokio_tcp_echo: an uppercase echo tcp server
  * run the server:
    * cargo run
  * for the client, use nc:
    * nc 127.0.0.1 6161 (Ctrl-C to exit)
* tokio_async_block_return:
  * type annotation in async closure
  * generic error type in order to use ? in async func
* tokio_tcp_tls: an uppercase tcp/tls server / client
  * TODO better doc 
  * gen certificate with certs/*.sh scripts (require openssl)
  * server_self_signed.rs / client_self_signed.rs: self signed certificate handling
  * server_ca_signed.rs / client_ca_signed.rs: certificate signed with local CA
  * main.rs / client_ca_signed_client_auth: cert signed with local CA + client auth (aka mTLS)
* tokio_future_pin:
  * Understanding Pin / Unpin + wrapping AsyncRead 
  * from fasterthanli.me tutorial

#### Tokio crates

* tokio_crate_loom_01: testing concurrent code under the C11 memory model
  * RUSTFLAGS="--cfg loom" cargo test --release

* tokio_future_01..05
  * from https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3

### Hyper (http)

* hyper_01_http_post: http server & client using hyper crate
  * run the server:
    * cargo run
  * run the client (or use curl, cmd line example in src files)
    * cargo run --example 03_client

## Various crates

* rust_crate_clap_O1: command line argument parsing
  * cargo run -- --hostname=localhost -p=2222 --kind=http3

* rust_crate_flume_01: mpmc queue (sync & async)
  * cargo run
  * Flume + queue peek (future): cargo run --example flume_peek

* rust_crate_serde_yaml_01: serde (serialization / deserialization) to yaml
  * basic example: cargo run
  * custom deserialization: cargo run --example custom de
  * validate_field (eg. non empty vec): cargo run --example validate_field
  * DE to enum: cargo run --example ipaddr_or_domain_name

* rust_crate_thiserror_anyhow_01: handling error easily using thiserror or anyhow crates
  * cargo run

* rust_crate_small_strings_01: from fasterthanli.me tutorial
  * crate: smol_str / smart_string (~ optimized String)
  * crate: argh (cmd line argument parsing lib)
  * custom allocator
  * alloc report to json file
  * TODO better doc -> cargo run -- report events.ldjson
