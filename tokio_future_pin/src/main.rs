use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // todo!();
        println!("MyFuture::poll()");

        // Note: pb here is that wake_by_ref() will be called a lot
        cx.waker().wake_by_ref(); // wake up the future
        Poll::Pending
    }
}

struct MyFuture1 {
    slept: bool,
}

impl Future for MyFuture1 {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Now use a thread to wake up the future after 1s
        println!("MyFuture1::poll()");
        match self.slept {
            false => {
                let waker = cx.waker().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(1));
                    waker.wake();
                });
                self.slept = true; // even if self.slept is set to true here, poll will only be called after wake()
                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}

struct MyFuture2 {
    // sleep: tokio::time::Sleep,
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl MyFuture2 {
    fn new() -> Self {
        MyFuture2 {
            // sleep: tokio::time::sleep(tokio::time::Duration::from_secs(1)),
            sleep: Box::pin(tokio::time::sleep(tokio::time::Duration::from_secs(1))),
        }
    }
}

impl Future for MyFuture2 {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Now use tokio timer instead of a thread
        println!("MyFuture2::poll()");

        // self.sleep.poll(cx) // does not work -> the method is available for `Pin<&mut Sleep>`
        // Pin::new(&mut self.sleep).poll(cx) // does not work unless sleep is in Box::pin
        self.sleep.as_mut().poll(cx) // self.sleep is already pinned (e.g. Box::pin(...))
    }
}

use futures::FutureExt; // allow poll_unpin()

struct MyFuture3 {
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl MyFuture3 {
    fn new() -> Self {
        MyFuture3 {
            sleep: Box::pin(tokio::time::sleep(tokio::time::Duration::from_secs(1))),
        }
    }
}

impl Future for MyFuture3 {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Same as MyFuture2 but using poll_unping from futures crate
        println!("MyFuture3::poll()");
        self.sleep.poll_unpin(cx)
    }
}

// an attempt to implement a SlowReader that implement the AsyncRead trait

use tokio::io::{AsyncRead, ReadBuf}; // trait: AsyncRead
use tokio::time::Instant;

struct SlowRead<R> {
    // reader: R
    reader: Pin<Box<R>>,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: Box::pin(reader),
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // todo!()

        // the following line will only work if reader is in Box::pin
        // self.reader.poll_read(cx, buf) // does not compile -> the method is available for `Pin<&mut R>`

        self.reader.as_mut().poll_read(cx, buf) // Not slower than R see SlowRead2
    }
}

struct SlowRead2<R> {
    reader: Pin<Box<R>>,
    sleep: Pin<Box<tokio::time::Sleep>>,
}

impl<R> SlowRead2<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: Box::pin(reader),
            sleep: Box::pin(tokio::time::sleep(Default::default())),
        }
    }
}

impl<R> AsyncRead for SlowRead2<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.sleep.poll_unpin(cx) {
            Poll::Ready(_) => {
                // sleep completes

                self.sleep
                    .as_mut()
                    .reset(Instant::now() + Duration::from_millis(25));
                self.reader.as_mut().poll_read(cx, buf) // Poll the reader
            }
            Poll::Pending => {
                // sleep has not completed yet
                Poll::Pending
            }
        }
    }
}

//

async fn app_fut2() {
    let fut2 = MyFuture2::new();
    fut2.await
}

async fn app_fut3() {
    let fut3 = MyFuture3::new();
    fut3.await
}

// use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
type AFnError = Box<dyn std::error::Error + Send + Sync>;
type AFnResult<T> = Result<T, AFnError>;

async fn app_slow_read() -> AFnResult<()> {
    let bufsize = 128 * 1024;
    let mut buf = vec![0u8; bufsize];

    let mut f = File::open("/dev/urandom").await?;
    let start = tokio::time::Instant::now();
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("1- Read {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("2- Slow'Read {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead2::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("2- Slow'Read2 {} bytes in {:?}", buf.len(), elapsed);

    Ok(())
}

fn main() {
    println!("Hello world");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();

    // let fut = MyFuture {};
    let fut1 = MyFuture1 { slept: false };
    // rt.block_on(fut);
    rt.block_on(fut1);

    rt.block_on(app_fut2());
    rt.block_on(app_fut3());

    rt.block_on(app_slow_read()).unwrap();
}
