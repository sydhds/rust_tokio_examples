use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::oneshot::Sender;

use scopeguard::defer;

use futures::{pin_mut, FutureExt};

fn main() {
    println!("Hello, world!");

    // 1. Forgetting about task cancellation
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(spawn_tasks(ToSpawn::MyTaskFixed2));

    println!("\n====================\n");
    // 2. select! and task cancellation
    println!("Use the unix command: 'nc 127.0.0.1 8081' to interact with this example");
    rt.block_on(spawn_select_task(ToSpawn2::SelectTask4))
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
                // the while loop (at line 57) will never return :-/
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

    // Note: Cannot do this otherwise got a runtime error like: async fn resumed after completion
    //       Check https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.accept (Cancel safety)
    // let tcp_accept = listener.accept();
    // tokio::pin!(tcp_accept);

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

    // Same as select_task2/3 but using Box::pin (Note: pin in the heap and not the stack)
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
