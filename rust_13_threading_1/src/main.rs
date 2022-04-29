
use std::thread;
use std::time::Duration;

fn main() {

    // Note: this code comes from a tutorial but
    // code is misleading - we want to launch X threads in //
    let handle = thread::spawn(|| {
        println!("- thread -");
        for i in 1..10 {
            println!("Hello world from thread {} {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    // Note: this is fixed here

    println!("Now using multiple threads...");
    let mut threads = vec![]; // where we store threads
    for i in 1..10 {
        threads.push(thread::spawn(move || {
            println!("Hello world from thread {} {:?}", i, thread::current().id());
            // this requires to use an unstable feature -> and unstable rust?
            // println!("Hello world from thread {} {}", i, thread::current().id().as_u64());
            thread::sleep(Duration::from_millis(1));
        }));
    }

    // wait for all thread
    for t in threads {
        let _ = t.join();
    }

    // map reduce with threads example

    println!("Map reduce with threads...");

    let mut threads_2 = vec![]; // where we store threads
    let data = "1981 6516 7436 6131 3215";

    let chunked_data = data.split_whitespace();
    // println!("chunked_data: {:?}", chunked_data);

    for (i, chunk) in chunked_data.enumerate() {
        threads_2.push(
            thread::spawn(move || -> u32 {
                // println!("Handling chunk {}: {}", i, chunk);
                let result: u32 = chunk
                    .chars()
                    .map(|c|
                        c.to_digit(10).expect("Should be a digit"))
                    .sum();
                // println!("result: {} {:?}", result, thread::current().id());
                result
        }));
    }

    // let final_result = threads_2.into_iter().map(|t| t.join().unwrap()).sum::<u32>();
    let final_result: u32 = threads_2.into_iter().map(|t| t.join().unwrap()).sum();
    println!("Final result: {}", final_result);

    let v = vec![1, 9, 8, 1, 6, 5, 1, 6, 7, 4, 3, 6, 6, 1, 3, 1, 3, 2, 1, 5];
    assert_eq!(final_result, v.iter().sum());
}
