use std::error::Error;

type AError = Box<dyn Error + Send + Sync>;
type AResult<T> = std::result::Result<T, AError>;

async fn my_sleep(delay: u64) -> AResult<()> {
    println!("Start to sleep...");
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
    println!("End sleep...");
    Ok(())
}

async fn run() -> AResult<()> {
    let delay = 100;
    let task = tokio::spawn(async move {
        my_sleep(delay).await?;
        // Note: from rust async-book chap 07 - 02: ? in async Blocks
        //       use turbofish to provide explicit type annotation
        // Ok::<(), Box<dyn Error + Send + Sync>>(())  // this works too
        Ok::<(), AError>(())
    });
    task.await?
}

async fn app_main() -> AResult<()> {
    let coro = tokio::spawn(run()); // this work
    let (res,) = tokio::join!(coro);
    res?
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(app_main()) {
        println!("Something went wrong: {}", e);
        std::process::exit(1); // exit program with value > 0 (usually en error in shell)
    }
}
