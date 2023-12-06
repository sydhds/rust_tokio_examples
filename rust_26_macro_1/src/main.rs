#![allow(unused_macros)]

use log::{debug, error, info, warn};

macro_rules! panic_or_warn {
    ($a: expr) => {{
        if cfg!(debug_assertions) {
            panic!($a)
        } else {
            warn!($a)
        }
    }};
}

macro_rules! panic_or {
    ($i: expr, $($b:expr), *) => {
        {
            if cfg!(debug_assertions) {
                panic!($($b,)*)
            } else {
                match $i {
                    "info" => info!($($b,)*),
                    "warn" => warn!($($b,)*),
                    "debug" => debug!($($b,)*),
                    _ => info!($($b,)*)
                }
            }
        }
    }
}

fn main() {
    simple_logger::SimpleLogger::new()
        .with_utc_timestamps()
        .init()
        .unwrap();

    info!("INFO");
    warn!("INFO");
    debug!("INFO");
    error!("ERROR");

    // uncomment and compile with either: cargo run --release / cargo run
    // panic_or_warn!("Something !!");

    let a: u32 = 42;
    let c: usize = 12356;
    let str1 = String::from("hello there!!");
    println!("Someting: {} | {}", a, c);

    panic_or!("debug", "Something !!");
    panic_or!("debug", "Something {} | {} | {}", a, c, str1);
}
