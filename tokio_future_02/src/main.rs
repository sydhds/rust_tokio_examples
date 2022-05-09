use std::error::Error;
use std::fmt::Debug;
// use std::fmt::Display;

// https://docs.rs/futures/latest/futures/
use futures::Future;
use futures::future::ok;
use futures::TryFutureExt;
use futures::future::err;

// From https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part2-8dd
// Adapted to futures 0.3

#[derive(Debug, Default)]
pub struct ErrorA {}

impl std::fmt::Display for ErrorA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ErrorA!")
    }
}

impl Error for ErrorA {
    fn description(&self) -> &str {
        "Description for Error A"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Debug, Default)]
pub struct ErrorB {}

impl std::fmt::Display for ErrorB {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ErrorB!")
    }
}

impl Error for ErrorB {
    fn description(&self) -> &str {
        "Description for Error B"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl From<ErrorB> for ErrorA {
    fn from(_e: ErrorB) -> ErrorA {
        ErrorA::default()
    }
}

impl From<ErrorA> for ErrorB {
    fn from(_e: ErrorA) -> ErrorB {
        ErrorB::default()
    }
}

fn fut_error_a() -> impl Future<Output=Result<(), ErrorA>> {
    err(ErrorA {})
}

fn fut_error_b() -> impl Future<Output=Result<(), ErrorB>> {
    err(ErrorB {})
}

fn my_fut_ref(s: &str) -> impl Future<Output=Result<&str, Box<dyn Error>>> {
    ok(s)
}

fn my_fut_ref_lifetime<'a>(s: &'a str) -> impl Future<Output=Result<&'a str, Box<dyn Error>>> {
    ok(s)
}

fn my_fut_ref_string<'a>(s: &'a str) -> impl Future<Output=Result<String, Box<dyn Error>>> + 'a {
    // as we annotate arguments we must also annotate the return
    my_fut_ref(s).and_then(|s| ok(format!("received: {}", s)))
}

fn main() {
    println!("Hello world");

    // use tokio to 'run' the future: my_fn_fut
    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(fut_error_a());
    println!("fut_error_A result: {:?}", retval);
    let retval = rt.block_on(fut_error_b());
    println!("fut_error_B result: {:?}", retval);

    // Note: to see compile error, comment: From<...> implementations
    // let chain_0 = fut_error_a().and_then(|_| fut_error_b()); // uncomment to see compil error

    // chain fut_error_A -> fut_error_B, Note: we need to convert ErrorA into ErrorB
    let chain_1 = fut_error_a().map_err(|e| {
        println!("mapping {:?} into ErrorB", e);
        ErrorB::default()
    }).and_then(|_| fut_error_b());

    let retval = rt.block_on(chain_1);
    println!("chain_1 result: {:?}", retval);

    // Using From<...> implementations we can do something like:
    let chain_2 = fut_error_a()
        .err_into()
        .and_then(|_| fut_error_b())
        .err_into()
        .and_then(|_| fut_error_a());

    let retval = rt.block_on(chain_2);
    println!("chain_2 result: {:?}", retval);

    // Note: with rustc 1.23 the following code would not compile
    //       with rustc 1.56+ all good!
    let my_str = "foo";
    let retval = rt.block_on(my_fut_ref(my_str));
    println!("my_fut_ref result: {:?}", retval);

    let retval = rt.block_on(my_fut_ref_lifetime(my_str));
    println!("my_fut_ref_lifetime result: {:?}", retval);

    let retval = rt.block_on(my_fut_ref_string(my_str));
    println!("my_fut_ref_string result: {:?}", retval);
}
