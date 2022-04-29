use std::sync::Arc;
use std::thread;


fn main() {

    let apple = Arc::new("the very same apple".to_string());
    let mut threads = vec![];

    for i in 1..10 {
        let apple_ = Arc::clone(&apple);
        println!("ref count: {}", Arc::strong_count(&apple));
        threads.push(thread::spawn(move || {
            println!("{}: {:?}", i, apple_);
        }));
    }

    for t in threads {
        t.join();
        // println!("ref count: {}", Arc::strong_count(&apple));
    }

}
