use futures::Future;
use std::{mem::swap, pin::Pin, task::Poll, time::Duration};
use tokio::{macros::support::poll_fn, time::sleep};

/*
 * run with:
 * RUST_BACKTRACE=1 cargo run --quiet --example 02_pin_crash
*/

#[tokio::main]
async fn main() {
    let mut sleep1 = sleep(Duration::from_secs(1)); // does not impl Unpin
    let mut sleep2 = sleep(Duration::from_secs(1)); // does not impl Unpin

    {
        // Pin sleep 1, as tokio::time::sleep does not implement Unpin
        // this is not safe (undefined behavior) to use it unpinned later
        // If tokio::time::sleep would iml Unpin using it later as unpinned would be totally ok
        let mut sleep1 = unsafe { Pin::new_unchecked(&mut sleep1) };
        println!("poll_fn ...");
        poll_fn(|cx| {
            let _ = sleep1.as_mut().poll(cx); // poll sleep1
            Poll::Ready(()) // resolve immediately
        })
        .await;
        println!("poll_fn ends!");
    }

    // let's now use sleep1 unpinned
    swap(&mut sleep1, &mut sleep2); // switch place between sleep1 & sleep2
    println!("swap done");
    println!("Now awaitin sleep1...");
    sleep1.await; // will crash on this line
    println!("Now awaiting sleep2...");
    sleep2.await;
    println!("All good!");
}
