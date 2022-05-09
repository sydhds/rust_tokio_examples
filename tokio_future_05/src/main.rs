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
use futures::{Stream, StreamExt, Sink, SinkExt};
use futures::task::Context;

// From https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part5---streams-5i8
// Stream trait for our future
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
            // println!("Not ready yet --> {:?}", self);

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

/*
impl Stream for WaitForIt {

    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    }
}
*/

// very basic Stream impl
#[derive(Debug)]
struct MyStream {
    current: u32,
    max: u32,
}

impl MyStream {
    pub fn new(max: u32) -> MyStream {
        MyStream { current: 0, max: max }
    }
}

impl Stream for MyStream {

    type Item = u32;

    // Note: possible return values
    //       * Poll::Pending: stream's next value is not ready yet
    //       * Poll::Ready(Some(val)): stream produced a value and may produce further values
    //       * Poll::Ready(None): stream has terminated
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {

        let max = self.max;

        match self.current {
            ref mut x if *x < max => {
                *x = *x + 1;
                Poll::Ready(Some(*x))
            }
            _ => Poll::Ready(None),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

}

// very basic Sink impl

use core::convert::Infallible;

#[derive(Debug)]
struct MySink {
    data: Vec<u32>,
    temp: u32,
}

impl MySink {
    pub fn new() -> MySink {
        MySink { data: vec![], temp: 0 }
    }
}

impl Sink<u32> for MySink {

    type Error = Infallible;

    fn poll_ready(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {

        // poll_ready: Attempts to prepare the Sink to receive a value
        println!("MySink poll ready");
        let new_len = self.data.len() + 1;
        self.data.reserve(new_len);
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: u32) -> Result<(), Self::Error> {

        // start_send: Begin the process of sending a value
        println!("Storing value: {}", item);
        self.temp = item;
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {

        // poll_flush:
        println!("MySink poll flush");
        let v = self.temp;
        self.data.push(v);
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {

        // poll_close:
        println!("MySink poll close");
        Poll::Ready(Ok(()))
    }

}


fn main() {
    println!("Hello world");

    let mut my_stream = MyStream::new(5);
    // This requires StreamExt trait ?
    /*
    let fut = my_stream.for_each(|num| {
        println!("num: {}", num);
        ok(())
    });
    */
    let fut = my_stream.next();
    println!("fut: {:?}", fut);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(fut);
    println!("retval: {:?}", retval);

    let mut my_stream_2 = MyStream::new(5);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(async {
        while let Some(value) = my_stream_2.next().await {
            println!("value: {}", value);
        }
    });

    let mut my_stream_3 = MyStream::new(11);
    let mut sink_1 = MySink::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let retval = rt.block_on(async {
        while let Some(value) = my_stream_3.next().await {
            sink_1.send(value).await;
        }
    });

    println!("sink_1: {:?}", sink_1);

}
