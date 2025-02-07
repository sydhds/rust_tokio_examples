use std::thread;
use std::time::Duration;

fn main() {
    // First example (from: https://marabos.nl/atomics/basics.html#scoped-threads)
    let numbers = vec![1, 2, 3];

    println!("[thread id: {:?}] main function", thread::current().id());
    println!("Launching scoped threads...");

    thread::scope(|s| {
        s.spawn(|| {
            println!(
                "[thread id: {:?}] length: {}",
                thread::current().id(),
                numbers.len()
            );
        });

        s.spawn(|| {
            for n in &numbers {
                println!("[thread id: {:?}] {n}", thread::current().id());
            }
        });
    });

    println!("Done launching scoped threads...");
}
