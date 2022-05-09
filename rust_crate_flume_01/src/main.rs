use flume;

type AFnError = Box<dyn std::error::Error + Send + Sync>;

async fn app_main() {

    //let (tx, rx) = flume::bounded(2);
    let (tx, rx) = flume::bounded(2);
    let rx2 = rx.clone(); // can clone rx as flume queue is mpmc (multi producer multi consumer)

    /*
    tx.send_async(5).await;
    tx.send_async(55).await;
    drop(tx);
    */

    tokio::spawn(async move {
        tx.send_async(5).await?;
        tx.send_async(55).await?;
        Ok::<(), AFnError>(())
    });

    // println!("tx len: {}", tx.len());
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

    let res = tokio::join!(rx.recv_async(), rx2.recv_async());
    println!("res: {:?}", res);
    let res2 = tokio::join!(rx.recv_async(), rx2.recv_async());
    println!("res2: {:?}", res2);
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(app_main());
}