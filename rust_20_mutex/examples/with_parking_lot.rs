use std::sync::Arc;
use std::thread;

use parking_lot::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // No need to unwrap() here with parking lot
        let mut num = m.lock();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    println!("[0] counter = {}", *counter.lock());
    let mut handles = vec![];

    for _ in 0..10 {
        // increment ref counting
        let counter = Arc::clone(&counter);
        // This line is ok too:
        // let counter = counter.clone();

        let handle = thread::spawn(move || {
            let mut c = counter.lock();
            *c += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("counter = {}", *counter.lock());
    println!("counter = {:?}", counter);
}
