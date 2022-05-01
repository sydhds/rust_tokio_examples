
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

### Tokio

* tokio_tcp_echo: an uppercase echo tcp server
  * run the server:
    * cargo run
  * for the client, use nc:
    * nc 127.0.0.1 6161 (Ctrl-C to exit)
* tokio_async_block_return:
  * type annotation in async closure
  * generic error type in order to use ? in async func

### Hyper (http)

* hyper_01_http_post: http server & client using hyper crate
  * run the server:
    * cargo run
  * run the client (or use curl, cmd line example in src files)
    * cargo run --example 03_client
