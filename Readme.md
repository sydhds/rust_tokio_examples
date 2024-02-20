## Intro

A bunch of very detailed and easy to read / understand examples written in Rust.

## Howto

* `cargo build --all && target/debug/rust_01_hello_world`

OR: 

* `cargo build --bin rust_01_hello_world`
* `cargo run --bin rust_01_hello_world`

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
* rust_20_...: thread + Mutex (+ Arc + [parking_lot](https://docs.rs/parking_lot/latest/parking_lot/) example)
* rust_21_...: rust basics - enum (enum with data)
* rust_22_...: impl Into for a struct
* rust_23_...: generic function / struct
* rust_24_...: implicit type conversion (coercion)
* rust_25_...: Path & PathBuf (filepath) + generic: (AsRef<Path>) / (Into<PathBuf>)
* rust_26_...: A simple macro
* rust_27_...: rust basics - Cow
* rust_28_...: impl IntoInter for a struct
* rust_29_...: Cell & RefCell + interior mutability from [this blog](https://ricardomartins.cc/2016/06/08/interior-mutability)
* rust_30_...: impl [Deref]() & [DerefMut]() trait for a struct 

### Tokio

* tokio_tcp_echo: an uppercase echo tcp server
  * run the server:
    * cargo run
  * for the client, use nc:
    * nc 127.0.0.1 6161 (Ctrl-C to exit)
* tokio_async_block_return:
  * type annotation in async closure
  * generic error type in order to use ? in async func
* tokio_async_common_mistakes:
  * rust async common mistakes from this [blog](https://www.qovery.com/blog/common-mistakes-with-rust-async/)
  * Use nc 127.0.0.1 8081 to interact with 2nd example
* tokio_tcp_tls: an uppercase tcp/tls server / client
  * gen certificate with certs/*.sh scripts (require openssl)
  * server_self_signed.rs / client_self_signed.rs: self signed certificate handling
  * server_ca_signed.rs / client_ca_signed.rs: certificate signed with local CA
  * main.rs / client_ca_signed_client_auth: cert signed with local CA + client auth (aka mTLS)
  * Check [Readme in tokio_tcp_tls](tokio_tcp_tls/Readme.md)

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

* rust_crate_clap_01: command line argument parsing using [clap](https://docs.rs/clap/latest/clap/)
  * cargo run -- --hostname 127.0.0.1 -k http3 "hello there!" -vvv
  * examples:
    * sub command (using [Derive](https://docs.rs/clap/latest/clap/_derive/index.html))
      * cargo run --example clap_subcommand -- --message hero stderr -p "Ola! new" -s 1234

* rust_crate_flume_01: [mpmc queue (sync & async)](https://docs.rs/flume/latest/flume/)
  * cargo run
  * Flume + queue peek (future):
    * cargo run --example flume_peek

* rust_crate_serde_yaml_01: [Serde](https://docs.rs/serde/latest/serde/) (serialization / deserialization) + [Serde yaml](https://github.com/dtolnay/serde-yaml)
  * basic example
    * cargo run
  * custom deserialization
    * cargo run --example custom de
  * validate_field (eg. non empty vec): 
    * cargo run --example validate_field
  * DE to enum: 
    * cargo run --example ipaddr_or_domain_name

* rust_crate_thiserror_anyhow_01: handling error easily using [thiserror](https://docs.rs/thiserror/latest/thiserror/) or [anyhow](https://docs.rs/anyhow/latest/anyhow/) crates
  * cargo run
  * RUST_BACKTRACE=1 cargo run
    * Full backtrace is printed when println! anyhow error

* rust_crate_nom_01: how to parse a jpeg file (binary file) using [nom](https://docs.rs/nom/latest/nom/)
  * wget https://upload.wikimedia.org/wikipedia/commons/3/3f/JPEG_example_flower.jpg
  * cargo run -- JPEG_example_flower.jpg

* rust_crate_rayon_01:
  * generate a ppm image and use rayon to process image data in //
    * cargo run 
      * xdg-open image.ppm
      * xdg-open image_gray.ppm

* rust_crate_small_strings_01: from [fasterthanli.me tutorial](https://fasterthanli.me/articles/small-strings-in-rust)
  * crate: [smol_str](https://docs.rs/smol_str/latest/smol_str/) / [smart_string](https://docs.rs/smartstring/latest/smartstring/)
    * String are stack allocated if small enough, heap allocated otherwise 
  * crate: argh (cmd line argument parsing lib)
  * custom allocator
  * alloc report to json file
  * How to run:
    * Read json using Serde + &str:
      * rm -f events.ldjson && ../target/debug/rust_crate_small_strings_01 sample --lib std 2> events.ldjson
    * Generate report:
      * ../target/debug/rust_crate_small_strings_01 report events.ldjson
    * Read json using Serde + smol_str:
      * rm -f events.ldjson && ../target/debug/rust_crate_small_strings_01 sample --lib smol 2> events.ldjson
    * Generate report:
      * ../target/debug/rust_crate_small_strings_01 report events.ldjson
    * Read json using Serde + smart_string:
      * rm -f events.ldjson && ../target/debug/rust_crate_small_strings_01 sample --lib smart 2> events.ldjson
    * Generate report:
      * ../target/debug/rust_crate_small_strings_01 report events.ldjson 

## Misc

* rustgdb_1: Debug rust program using rust-gdb
  * Check [Readme](rustgdb_1/Readme.md)

* [BROKEN] rust_crate_smol: async code using to [smol](https://docs.rs/smol/latest/smol/)
  * From articles: 
    * https://notgull.net/why-you-want-async/
    * https://notgull.net/futures-concurrency-in-smol/