use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 01: simple example
    let m = Mutex::new(5);

    {
        // Need to unwrap() after lock() because if another thread while holding the lock panic
        // the mutex cannot be used anymore
        let mut num = m.lock().unwrap();
        println!("is poisoned?: {}", Mutex::is_poisoned(&m));
        *num = 6;
        // as num is a MutexGuard, it will now goes out of scope and Mutex will be released
    }

    println!("m = {:?}", m);

    // 02: This code will not compile => cannot move ownership of Mutex into multiple threads
    /*
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut m = counter.lock().unwrap();
            *m += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join();
    }
    */

    let counter = Arc::new(Mutex::new(0));
    println!("[0] counter = {}", *counter.lock().unwrap());
    let mut handles = vec![];

    for _ in 0..10 {
        // increment ref counting
        let counter = Arc::clone(&counter);
        // This line is ok too:
        // let counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut c = counter.lock().unwrap();
            *c += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter = {}", *counter.lock().unwrap());
    println!("counter = {:?}", counter);
}
