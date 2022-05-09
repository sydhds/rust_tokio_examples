use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use std::time::Duration;

// an attempt to implement a SlowReader that implement the AsyncRead trait

use tokio::io::{AsyncRead, ReadBuf}; // trait: AsyncRead
use tokio::time::Instant;
use futures::FutureExt; // allow poll_unpin()

// Almost the same as SlowRead2 in main.rs

struct SlowRead2<R> {
    // reader: Pin<Box<R>>,
    reader: R,
    sleep: Pin<Box<tokio::time::Sleep>>
}

impl<R> SlowRead2<R> {
    fn new(reader: R) -> Self {
        Self {
            // reader: Box::pin(reader),
            reader: reader,
            sleep: Box::pin(tokio::time::sleep(Default::default())),
        }
    }
}

// Note: restrict R to AsyncRead + Unpin (as recommanded by the compiler
// In our example, tokio::fs::File does implement Unpin
impl<R> AsyncRead for SlowRead2<R> where R: AsyncRead + Unpin {

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {

        match self.sleep.poll_unpin(cx) {
            Poll::Ready(_) => { // sleep completes

                self.sleep.
                    as_mut()
                    .reset(Instant::now() + Duration::from_millis(25));

                // self.reader.as_mut().poll_read(cx, buf) // Poll the reader // does not work if reader is not wrapped in Box::pin
                Pin::new(&mut self.reader).poll_read(cx, buf) // recommandation from the compiler
            },
            Poll::Pending => { // sleep has not completed yet
                Poll::Pending
            },
        }
    }
}


// SlowRead3 cannot compile: SlowRead3 does not impl Unpin (because field: sleep does not impl Unpin)
// If SlowRead3 does not impl Unpin: cannot do Pin::new(&mut self.reader)

/*
// Same as SlowRead2 but we would like to avoid Box::pin for self.sleep too
struct SlowRead3<R> {
    reader: R,
    sleep: tokio::time::Sleep,  // Does not impl: Unpin
}

impl<R> SlowRead3<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R> AsyncRead for SlowRead3<R> where R: AsyncRead + Unpin {

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {

        // let sleep = Pin::new(&mut self.sleep); // does not compile
        // let sleep = Pin::new_unchecked(&mut self.sleep); // does not compile
        let sleep = unsafe { Pin::new_unchecked(&mut self.sleep); };  // new_unchecked() is unsafe

        match sleep.poll(cx) {
            Poll::Ready(_) => { // sleep completes
                // let sleep = Pin::new(&mut self.sleep); // does not compile
                let sleep = Pin::new_unchecked(&mut self.sleep); // new_unchecked() is unsafe
                sleep.reset(Instant::now() + Duration::from_millis(25));
                Pin::new(&mut self.reader).poll_read(cx, buf)
            },
            Poll::Pending => { // sleep has not completed yet
                Poll::Pending
            },
        }
    }
}
*/

// Same as SlowRead2 but we would like to avoid Box::pin for self.sleep too
struct SlowRead4<R> {
    reader: R,
    sleep: tokio::time::Sleep,
}

impl<R> SlowRead4<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R> AsyncRead for SlowRead4<R> where R: AsyncRead + Unpin {

    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {

        let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };

        match sleep.poll(cx) {
            Poll::Ready(_) => { // sleep completes
                let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };
                sleep.reset(Instant::now() + Duration::from_millis(25));
                // Pin::new(&mut self.reader).poll_read(cx, buf)
                let reader = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.reader) };
                reader.poll_read(cx, buf)
            },
            Poll::Pending => { // sleep has not completed yet
                Poll::Pending
            },
        }
    }
}

// Same as SlowRead4 but only one unsafe block
struct SlowRead5<R> {
    reader: R,
    sleep: tokio::time::Sleep,
}

impl<R> SlowRead5<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R> AsyncRead for SlowRead5<R> where R: AsyncRead + Unpin {

    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {

        let (mut sleep, reader) = unsafe {
            let this = self.get_unchecked_mut();
            (
                Pin::new_unchecked(&mut this.sleep),
                Pin::new_unchecked(&mut this.reader),
            )
        };

        match sleep.as_mut().poll(cx) {
            Poll::Ready(_) => { // sleep completes
                sleep.reset(Instant::now() + Duration::from_millis(25));
                reader.poll_read(cx, buf)
            },
            Poll::Pending => { // sleep has not completed yet
                Poll::Pending
            },
        }
    }
}

// Same as SlowRead5 but without unsafe block (thanks to pin-project)

use pin_project::pin_project;

#[pin_project]
struct SlowRead6<R> {
    #[pin]
    reader: R,
    #[pin]
    sleep: tokio::time::Sleep,
}

impl<R> SlowRead6<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R> AsyncRead for SlowRead6<R> where R: AsyncRead + Unpin {

    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {

        // project() will return a:
        // struct SlowReadProjected<'a, R> {
        //   reader: Pin<&'a mut R>,
        //   writer: Pin<&'a mut Sleep>,
        // }
        let mut this = self.project();

        match this.sleep.as_mut().poll(cx) {
            Poll::Ready(_) => { // sleep completes
                this.sleep.reset(Instant::now() + Duration::from_millis(25));
                this.reader.poll_read(cx, buf)
            },
            Poll::Pending => { // sleep has not completed yet
                Poll::Pending
            },
        }
    }
}

// as an exercise, try to implement a TimedFuture that will return result of wrapped future + duration
// from cloudfare article

#[derive(Debug)]
struct TimedFuture<F> {
    fut: Pin<Box<F>>,
    start: Option<tokio::time::Instant>,
    // elapsed: Option<tokio::time::Instant>,
}

impl<F> TimedFuture<F> {

    fn new(f: F) -> Self where F: Future {

        TimedFuture {
            fut: Box::pin(f),
            start: None,
            // elapsed: None,
        }
    }
}

impl<F: Future> Future for TimedFuture<F> {
    type Output = (F::Output, tokio::time::Duration);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        self.start.get_or_insert_with(tokio::time::Instant::now);
        // println!("self.start: {:?}", self.start);

        // initial code: ignoring about get_or_insert_with...
        /*
        let start = match self.start {
            None => {
                self.start = Some(tokio::time::Instant::now());
                self.start.unwrap()
            },
            _ => self.start.unwrap(),
        };
        */

        match self.fut.as_mut().poll(cx) {
            Poll::Ready(o) => {
                println!("[f] self.start: {:?}", self.start);
                Poll::Ready(
                    (o, self.start.unwrap().elapsed())
                    //(o, start.elapsed())
                )
            },
            Poll::Pending => {
                Poll::Pending
            },
        }
    }
}

#[pin_project]
struct TimedFuture2<F> {
    #[pin]
    fut: F,
    start: Option<tokio::time::Instant>,
}

impl<F> TimedFuture2<F> {

    fn new(f: F) -> Self where F: Future {

        TimedFuture2 {
            fut: f,
            start: None,
        }
    }
}

impl<F: Future> Future for TimedFuture2<F> {
    type Output = (F::Output, tokio::time::Duration);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut this = self.project();
        this.start.get_or_insert_with(tokio::time::Instant::now);

        match this.fut.as_mut().poll(cx) {
            Poll::Ready(o) => {
                Poll::Ready(
                    (o, this.start.unwrap().elapsed())
                )
            },
            Poll::Pending => {
                Poll::Pending
            },
        }
    }
}

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

    let mut f = SlowRead2::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("2- Slow'Read2 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead4::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("3- Slow'Read4 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead5::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("4-1- Slow'Read5 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead5::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    pin_utils::pin_mut!(f); // use pin-utils crate: pin an owned value to the stack, shadowing its previous name
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("4-2- Slow'Read5 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead5::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    tokio::pin!(f); // same as when we wre using pin_utils but this time with the macro from tokio
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("4-3- Slow'Read5 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead5::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    let mut f = Box::pin(f); // same but Box::pin the whole SlowRead to the heap
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("4-4- Slow'Read5 {} bytes in {:?}", buf.len(), elapsed);

    // Here we will pass a mut ref of File to SlowRead - allow to reuse File
    let mut f = File::open("/dev/urandom").await?;
    let sr = SlowRead5::new(&mut f);
    let start = tokio::time::Instant::now();
    tokio::pin!(sr);
    sr.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    f.read_exact(&mut buf).await?; // reuse File here
    println!("4-5- Slow'Read5 {} bytes in {:?}", buf.len(), elapsed);

    let mut f = SlowRead6::new(File::open("/dev/urandom").await?);
    let start = tokio::time::Instant::now();
    let mut f = Box::pin(f); // same but Box::pin the whole SlowRead to the heap
    f.read_exact(&mut buf).await?;
    let elapsed = start.elapsed();
    println!("5- Slow'Read6 {} bytes in {:?}", buf.len(), elapsed);


    // timed future

    let tf = TimedFuture::new(
        tokio::time::sleep(tokio::time::Duration::from_secs(1)),
    );
    println!("tf: {:?}", tf);
    let tf_res = tf.await;
    println!("tf result: {:?}", tf_res);

    let tf2 = TimedFuture2::new(
        tokio::time::sleep(tokio::time::Duration::from_secs(1)),
    );
    // println!("tf2: {:?}", tf2);
    let tf2_res = tf2.await;
    println!("tf2 result: {:?}", tf2_res);

    Ok(())
}

fn main() {
    println!("Hello world");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();

    rt.block_on(app_slow_read());
}
