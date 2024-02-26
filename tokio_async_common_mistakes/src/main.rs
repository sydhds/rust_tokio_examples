use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::oneshot::Sender;
use tokio::sync::{Semaphore, SemaphorePermit};

use scopeguard::defer;

use futures::{pin_mut, FutureExt};
use tokio::io::AsyncWriteExt;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 1. Forgetting about task cancellation
    rt.block_on(spawn_tasks(ToSpawn::MyTaskFixed2));

    // 2. select! and task cancellation
    println!("\n====================\n");
    println!("Use the unix command: 'nc 127.0.0.1 8081' to interact with this example");
    rt.block_on(spawn_select_task(ToSpawn2::SelectTask4));

    // 3. Not using sync Mutex
    println!("\n====================\n");
    rt.block_on(spawn_mutex_tasks());

    // 4. Holding RAII/guard object across await point
    println!("\n====================\n");
    rt.block_on(spawn_pool_tasks());

    // 5. Future starvation
    println!("\n====================\n");
    rt.block_on(spawn_progress_tasks());
}

//
// 1. Forgetting about task cancellation

#[allow(dead_code)]
enum ToSpawn {
    MyTask,
    MyTaskWithTimeout,
    MyTaskFixed1,
    MyTaskFixed2,
}

async fn spawn_tasks(to_spawn: ToSpawn) {
    let task_counter = Arc::new(AtomicUsize::new(0));

    println!("Launching tasks....");
    for _ in 0..100 {
        let task_counter = task_counter.clone();

        match to_spawn {
            ToSpawn::MyTask => {
                tokio::spawn(my_task(task_counter));
            }
            ToSpawn::MyTaskWithTimeout => {
                // Exhibit the issue with function my task (timeout will cancel my_task)
                // the while loop (at line 71) will never return :-/
                tokio::spawn(tokio::time::timeout(
                    Duration::from_millis(10),
                    my_task(task_counter),
                ));
            }
            ToSpawn::MyTaskFixed1 => {
                tokio::spawn(tokio::time::timeout(
                    Duration::from_millis(10),
                    my_task_fix1(task_counter),
                ));
            }
            ToSpawn::MyTaskFixed2 => {
                tokio::spawn(tokio::time::timeout(
                    Duration::from_millis(10),
                    my_task_fix2(task_counter),
                ));
            }
        }
    }

    while task_counter.load(Ordering::Relaxed) > 0 {
        println!("Waiting for task counter to be zero....");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn my_task(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);

    // At any await, the task can be cancelled
    let _ = tokio::time::sleep(Duration::from_millis(200)).await;

    // This instruction may never be run, because task has been cancelled.
    task_counter.fetch_sub(1, Ordering::Relaxed);
}

async fn my_task_fix1(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);

    // Call our closure when _guard is dropped
    // Note: The only guarantee we have about future completion, is that at some point the future will be dropped.
    let _guard = scopeguard::guard((), |_| {
        println!("[my_task_fix1] Decr counter...");
        task_counter.fetch_sub(1, Ordering::Relaxed);
    });

    let _ = tokio::time::sleep(Duration::from_millis(200)).await;
}

async fn my_task_fix2(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);

    // defer! is equivalent of the scopeguard::guard (see my_task_fix1)
    defer! {
        println!("[my_task_fix2] Decr counter...");
        task_counter.fetch_sub(1, Ordering::Relaxed);
    }

    let _ = tokio::time::sleep(Duration::from_millis(200)).await;
}

//
// 2. select! and task cancellation

#[allow(dead_code)]
enum ToSpawn2 {
    SelectTask1, // no reuse
    SelectTask2, // tokio::pin!
    SelectTask3, // pin_mut!
    SelectTask4, // Box::pin
}

async fn spawn_select_task(to_spawn: ToSpawn2) {
    let (oneshot_sender, oneshot_recv) = tokio::sync::oneshot::channel::<()>();

    match to_spawn {
        ToSpawn2::SelectTask1 => {
            tokio::spawn(select_task_1(oneshot_sender));
        }
        ToSpawn2::SelectTask2 => {
            tokio::spawn(select_task_2(oneshot_sender));
        }
        ToSpawn2::SelectTask3 => {
            tokio::spawn(select_task_3(oneshot_sender));
        }
        ToSpawn2::SelectTask4 => {
            tokio::spawn(select_task_4(oneshot_sender));
        }
    }
    println!("Select task has been spawned...");

    let wait_for = Duration::from_secs(7);
    println!("Now waiting for {} secs...", wait_for.as_secs());
    let _ = tokio::time::sleep(wait_for).await;
    // If task is not finished after 5 sec, abort it (by dropping the receiver)
    println!("Aborting after {} secs...", wait_for.as_secs());
    drop(oneshot_recv);
}

async fn select_task_1(mut should_abort: Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    // Not optimal - async tasks are created at each loop iteration
    loop {
        println!("Starting loop (select task 1)...");
        select! {
            biased;

            _ = should_abort.closed() => {
                println!("Aborting task");
                return;
            }
            _ = listener.accept() => {
                println!("A tcp connection is there!!");
            }
        }
    }
}

async fn select_task_2(mut should_abort: Sender<()>) {
    // Setup tcp server
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    // Using the same future in multiple select! expressions can be done by passing a reference to the future.
    // Doing so requires the future to be Unpin. A future can be made Unpin by either using Box::pin or stack pinning.
    let should_abort = should_abort.closed();
    tokio::pin!(should_abort);

    // Note: Cannot do this otherwise got a runtime error like: async fn resumed after completion
    //       Check https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.accept (Cancel safety)
    // let tcp_accept = listener.accept();
    // tokio::pin!(tcp_accept);

    loop {
        println!("Starting loop (select_task_2)...");
        select! {
            biased;

            _ = &mut should_abort => {
                println!("Aborting task");
                return;
            }
            _ = listener.accept() => {
                println!("A tcp connection is there!!");
            }
        }
    }
}

async fn select_task_3(mut should_abort: Sender<()>) {
    // Setup tcp server
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    // Same as select_task2 but using crate futures
    let should_abort = should_abort.closed().fuse();
    pin_mut!(should_abort);

    loop {
        println!("Starting loop (select_task_3)...");
        select! {
            biased;

            _ = &mut should_abort => {
                println!("Aborting task");
                return;
            }
            _ = listener.accept() => {
                println!("A tcp connection is there!!");
            }
        }
    }
}

async fn select_task_4(mut should_abort: Sender<()>) {
    // Setup tcp server
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    // Same as select_task2/3 but using Box::pin (Note: pin on the heap and not on the stack)
    let should_abort = should_abort.closed();
    let mut should_abort_boxed = Box::pin(should_abort);

    loop {
        println!("Starting loop (select_task_4)...");
        select! {
            biased;

            _ = &mut should_abort_boxed => {
                println!("Aborting task");
                return;
            }
            _ = listener.accept() => {
                println!("A tcp connection is there!!");
            }
        }
    }
}

//
// 3. Not using sync Mutex

async fn spawn_mutex_tasks() {
    // let (oneshot_sender_1, oneshot_recv_1) = tokio::sync::oneshot::channel::<()>();
    // let (oneshot_sender_2, oneshot_recv_2) = tokio::sync::oneshot::channel::<()>();

    let h1 = tokio::spawn(my_mutex_task());
    let h2 = tokio::spawn(my_mutex_task_fix1());

    let res1 = h1.await;
    let res2 = h2.await;

    println!("res 1: {:?}", res1);
    println!("res 2: {:?}", res2);
}

async fn my_mutex_task() -> BTreeMap<i32, i32> {
    let mut set = tokio::task::JoinSet::new();

    // Note: this code is ok but here a std::sync::Mutex can be used as well
    //       DO NOT USE a sync::Mutex if you are holding the guard across await point.
    let workers = Arc::new(tokio::sync::Mutex::new(BTreeMap::new()));

    for i in 1..10 {
        let workers = workers.clone();
        set.spawn(async move {
            let mut workers = workers.lock().await;
            workers.insert(i, i);
            i
        });
    }

    while let Some(res) = set.join_next().await {
        let idx = res.unwrap();
        println!("Task (idx: {}) is finished...", idx);
    }

    // println!("workers: {:?}", workers);
    let res = workers.lock().await;
    res.clone()
}

async fn my_mutex_task_fix1() -> BTreeMap<i32, i32> {
    let workers = Arc::new(std::sync::Mutex::new(BTreeMap::new()));

    let mut handles = vec![];
    for i in 1..10 {
        let workers = workers.clone();
        handles.push(tokio::spawn(async move {
            let mut workers = workers.lock().unwrap();
            workers.insert(i, i);
            i
        }));
    }

    let finished_task_ids = futures::future::join_all(handles).await;
    println!("Finished task ids: {:?}", finished_task_ids);

    let res = workers.lock().unwrap();
    res.clone()
}

//
// 4. Holding RAII/guard object across await point

struct DummyConnectionPool {
    // max_size: u32,
    semaphore: Semaphore,
}

impl DummyConnectionPool {
    const fn new(max_size: u32) -> Self {
        Self {
            semaphore: Semaphore::const_new(max_size as usize),
        }
    }

    async fn get(&self) -> DummyConnection {
        let permit = self.semaphore.acquire().await.unwrap();
        DummyConnection { permit }
    }
}

#[allow(dead_code)]
struct DummyConnection<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> DummyConnection<'a> {
    async fn fetch(&self) -> Vec<i32> {
        vec![42, 42, 42]
    }

    async fn put(&self) {
        // Simulate put
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}

static CNX_POOL: DummyConnectionPool = DummyConnectionPool::new(1);

async fn spawn_pool_tasks() {
    let mut set = tokio::task::JoinSet::new();
    for i in 1..3 {
        let cnx_pool = &CNX_POOL;
        set.spawn(my_pool_task(cnx_pool, i));
    }

    while let Some(res) = set.join_next().await {
        let idx = res.unwrap();
        println!("Task (idx: {}) is finished...", idx);
    }

    println!("=== now with my_pool_task_fix1...");

    let mut set2 = tokio::task::JoinSet::new();
    for i in 1..3 {
        let cnx_pool = &CNX_POOL;
        set2.spawn(my_pool_task_fix1(cnx_pool, i));
    }

    while let Some(res) = set2.join_next().await {
        let idx = res.unwrap();
        println!("Task (idx: {}) is finished...", idx);
    }
}

async fn my_pool_task(pool: &DummyConnectionPool, i: i32) -> i32 {
    println!("[task {}] get", i);
    // Note: cnx will only be dropped at the end of this function
    //       because between fetch & put there is some delay the pool cannot return some connection
    let cnx = pool.get().await;
    println!("[task {}] fetch", i);
    let rules_ret = cnx.fetch().await;

    println!("[task {}] process rule", i);
    for _rule in rules_ret {
        // Simulate process_rule here with tokio sleep
        // process_rule(rule).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("[task {}] put", i);
    cnx.put().await;
    i
}

async fn my_pool_task_fix1(pool: &DummyConnectionPool, i: i32) -> i32 {
    println!("[task {}] get & fetch", i);
    let rules_ret = {
        // Note: use scope to avoid holding the connection for too long
        let cnx = pool.get().await;
        cnx.fetch().await
    };

    println!("[task {}] process rule", i);
    for _rule in rules_ret {
        // Simulate process_rule here with tokio sleep
        // process_rule(rule).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    {
        let cnx = pool.get().await;
        println!("[task {}] put", i);
        cnx.put().await
    };
    i
}

//
// 5. Future progress starvation

async fn spawn_progress_tasks() {
    let mut stdout = tokio::io::stdout();

    let (sender1, receiver1) = tokio::sync::mpsc::unbounded_channel();

    let h = tokio::spawn(my_progress_task(receiver1));

    sender1.send(800).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    sender1.send(900).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    sender1.send(5000).unwrap();

    // let _ = stdout
    //     .write_all(b"Now waiting for 7 seconds before aborting...\n")
    //     .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
    h.abort();

    let _ = stdout
        .write_all(b"\n=== now with my_progress_task_fix1...\n")
        .await;

    let (sender2, receiver2) = tokio::sync::mpsc::unbounded_channel();
    let h = tokio::spawn(my_progress_task_fix1(receiver2));

    sender2.send(800).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    sender2.send(900).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    sender2.send(5000).unwrap();

    // let _ = stdout
    //     .write_all(b"Now waiting for 7 seconds before aborting...\n")
    //     .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
    h.abort();
}

async fn my_progress_task(mut work: tokio::sync::mpsc::UnboundedReceiver<u64>) {
    let mut heartbeat = tokio::time::interval(tokio::time::Duration::from_secs(1));
    let mut stdout = tokio::io::stdout();

    loop {
        select! {
           biased;

            _ = heartbeat.tick() => {
                // send heartbeat
                let now = tokio::time::Instant::now();
                let _res = stdout.write_all(
                    format!("[{:?}] Sending heartbeat...\n", now).as_bytes()
                ).await;
            }
            msg = work.recv() => {
                let _ = stdout.write_all(
                    format!("Got a msg ({:?})\n", msg).as_bytes()
                ).await;
                // Simulate process_work with tokio sleep
                // process_work(msg); // making it async don't change the issue
                // Note: this sleep will prevent heartbeat to send a heartbeat if the sleeping time
                //       is too high (this simulates a process work that is higher than the tick interval)
                // Note 2: if you check the logs, you will notice some delay in heartbeat messages
                tokio::time::sleep(tokio::time::Duration::from_millis(msg.unwrap())).await;
                let _ = stdout.write_all(
                    format!("End of process work ({:?})\n", msg).as_bytes()
                ).await;
            }
        }
    }
}

async fn my_progress_task_fix1(mut work: tokio::sync::mpsc::UnboundedReceiver<u64>) {
    let mut heartbeat = tokio::time::interval(tokio::time::Duration::from_secs(1));
    let mut stdout = tokio::io::stdout();

    loop {
        select! {
           biased;

            _ = heartbeat.tick() => {
                // send heartbeat
                let now = tokio::time::Instant::now();
                let _res = stdout.write_all(
                    format!("[{:?}] Sending heartbeat...\n", now).as_bytes()
                ).await;
            }
            msg = work.recv() => {
                let _ = stdout.write_all(
                    format!("Got a msg ({:?})\n", msg).as_bytes()
                ).await;
                // Simulate process_work with tokio sleep
                // process_work(msg); // making it async don't change the issue
                // Note: using spawn blocking we do not block the heartbeat...
                let _h = tokio::task::spawn_blocking(move || {
                    std::thread::sleep(std::time::Duration::from_millis(msg.unwrap_or_default()));
                    println!("End of process work {:?}\n", msg);
                });
            }
        }
    }
}
