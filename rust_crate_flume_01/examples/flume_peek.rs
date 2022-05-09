use flume;
use futures::stream::{self, StreamExt};
use futures::pin_mut;
use rand::Rng;

// cargo run --example flume_peek

#[derive(Debug, Clone)]
struct Message {
    foo: String,
    bar: u32,
    baz: Vec<u8>,
}

const MESSAGE_COUNT: u32 = 9;

async fn feed_queue(tx: flume::Sender<Message>) {

    //todo!()

    let mut m = Message {
        foo: "feed_queue".into(),
        bar: 0,
        baz: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
    };

    for i in 0..MESSAGE_COUNT {
        // println!("i: {}", i);
        let m_ = m.clone();
        tx.send_async(m_).await;
        println!("Just sent a msg with index: {:?}", i);
        //tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        //tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        m.bar += 1;
    }

}

async fn sink_queue(rx: flume::Receiver<Message>) {

    let rx_ = rx.clone();
    let rxs = rx.into_stream().peekable();
    // println!("rxs: {:?}", rxs);

    // let stream = stream::iter(vec![1, 2, 3]);
    // let stream_p = stream.peekable();
    // println!("stream_p: {:?}", stream_p);

    pin_mut!(rxs);
    loop {

        let msg_: Option<&Message> = rxs.as_mut().peek().await; // get reference
        println!("peek! {:?}", rx_.len());
        if msg_.is_none() {
            // stream has ended, break the loop...
            break; }
        println!("msg_ bar: {:?}", msg_.unwrap().bar);

        // Do something, here we just generate random bool
        let has_failed: bool = rand::random();

        if has_failed {
            println!("Failed operation, continue...");
            continue;
        } else {
            println!("Operation succeeded, will pop Message from queue...");
        }

        let res_next = rxs.next().await; // pop message
        if res_next.is_none() {
            // stream has ended, break the loop
            break;
        }
        println!("next! {:?}", rx_.len());
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

async fn app_main() {

    let (tx, rx) = flume::bounded::<Message>(2);

    println!("tx: {:?}", tx);
    println!("rx: {:?}", rx);
    let rx_ = rx.clone();

    let c1 = tokio::spawn(feed_queue(tx));
    let c2 = tokio::spawn(sink_queue(rx));

    let res = tokio::join!(c1, c2);

    println!("rx size: {}", rx_.len());
}

fn main() {

    println!("Hello there!");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(app_main());
}
