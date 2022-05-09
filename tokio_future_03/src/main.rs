use std::error::Error;
use std::fmt::Debug;
// use std::fmt::Display;
use core::pin::Pin;

// https://docs.rs/futures/latest/futures/
use futures::Future;
use futures::future::ok;
use futures::TryFutureExt;
use futures::future::err;
use futures::task::Poll;
use chrono::{DateTime, Duration, Utc};

// From https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part3---thereactor-433
// implement a future from scratch, run 2 of them concurrently, run 2 of them and retrieve result for the 1st one that end
// Adapted to futures 0.3

#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}

impl WaitForIt {

    pub fn new(message: String, delay: Duration) -> Self {
        Self {
            message: message,
            until: Utc::now() + delay,
            polls: 0
        }
    }
}

impl Future for WaitForIt {

    // asociated types: what future will return upon completion or error
    type Output = Result<String, Box<dyn Error>>;

    // see doc.rs/futures/0.3.17/futures/future/trait.Future.html
    fn poll(mut self: Pin<&mut Self>, cx: &mut futures::task::Context<'_>) -> Poll<Self::Output> {

        let now = Utc::now();
        if self.until < now {
            // Tell the async runtime we are done: Async::Ready
            Poll::Ready(
                Ok(format!("{} after {} polls!", self.message, self.polls))
            )
        } else {
            self.polls += 1;
            println!("Not ready yet --> {:?}", self);

            // unpark the task
            // When Poll::Pending is returned, function is 'parked'
            // The runtime does not poll a parked function
            // so we need to unpark it (here manually to this example)

            //futures::task::current().notify(); // old version of futures
            cx.waker().wake_by_ref();  // futures 0.3

            // Tell the async runtime we are not ready yet: Async::NotReady
            Poll::Pending
        }
    }
}


fn main() {
    println!("Hello world");

    // build on our own future and run it with tokio runtime
    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    println!("wfi_1: {:?}", wfi_1);

    // use tokio to 'run' the future: wfi_1
    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(wfi_1);
    println!("retval: {:?}", retval);

    // join 2 futures and run them in concurrently
    let wfi_2 = WaitForIt::new("I'm done first!".into(), Duration::seconds(1));
    let wfi_3 = WaitForIt::new("I'm done 2nd!".into(), Duration::seconds(2));

    let v = vec![wfi_2, wfi_3];
    let sel = futures::future::join_all(v);
    let retval = rt.block_on(sel);
    println!("retval: {:?}", retval);

    // same as previous but return the first one to complete

    let wfi_4 = WaitForIt::new("I'm done 2nd! 4".into(), Duration::seconds(2));
    let wfi_5 = WaitForIt::new("I'm done first! 5".into(), Duration::seconds(1));

    let v = vec![wfi_4, wfi_5];
    let sel = futures::future::select_all(v);
    let retval = rt.block_on(sel);
    println!("retval: {:?}, pending futures count: {:?}, pending futures list: {:?}", retval.0, retval.1, retval.2);
}
