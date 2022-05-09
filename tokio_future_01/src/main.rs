use std::error::Error;
use std::fmt::Debug;

// https://docs.rs/futures/latest/futures/
use futures::Future;
use futures::future::ok;
use futures::TryFutureExt;

// From https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part1-3k3
// Adapted to futures 0.3

fn my_fn() -> Result<u32, Box<dyn Error>> {
    Ok(100)
}

// Note: Debug is not mandatory
fn my_fn_fut() -> impl Future<Output = Result<u32, Box<dyn Error>>> + Debug {
    futures::future::ok(9)  // or: ok(9)
}

fn my_fn_squared(i: u32) -> impl Future<Output = Result<u32, Box<dyn Error>>> {
    ok(i * i)
}

fn fn_plain(i: u32) -> u32
{
    i - 3
}

fn fut_generic_own<T>(a1: T, a2: T) -> impl Future<Output = Result<T, Box<dyn Error>> >
    where T: std::cmp::PartialOrd
{
    if a1 < a2 {
        ok(a1)
    } else {
        ok(a2)
    }
}

fn main() {
    println!("Hello world");
    println!("my_fn: {:?}", my_fn());
    println!("my_fn_fut0: {:?}", my_fn_fut());

    // use tokio to 'run' the future: my_fn_fut
    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(my_fn_fut());
    println!("my_fn_fut result: {:?}", retval.unwrap());

    // now chain futures: my_fn_fut -> my_fn_squared
    let chained_future = my_fn_fut().and_then(|retval| my_fn_squared(retval));
    // FIXME: not sure why we do not need: chained_future() ??
    let retval = rt.block_on(chained_future);
    println!("chained_future result: {:?}", retval.unwrap());

    // chain future + sync: my_fn_fut() -> fn_plain() -> my_fn_squared
    let chained_future_2 = my_fn_fut().and_then(|retval| {
        let retval2 = fn_plain(retval);
        my_fn_squared(retval2)
    });
    let retval = rt.block_on(chained_future_2);
    println!("chained_future_2 result: {:?}", retval.unwrap());

    // same as previous but using done()
    // FIXME: futures::done exits in version 0.1.3 but not in current version (0.3.17) anymore
    /*
    let chained_future_3 = my_fn_fut().and_then(|retval| {
        done(fn_plain(retval)).and_then(|retval2| my_fn_squared(retval2))
    });

    let retval = rt.block_on(chained_future_3);
    println!("chained_future_3 result: {:?}", retval.unwrap());
    */

    let retval = rt.block_on(fut_generic_own("Foo", "Baz"));
    println!("fut_generic_own result: {:?}", retval.unwrap());

}