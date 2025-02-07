use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    // Communicating between threads using a channel (MPSC: Multi producer - Single consumer)
    // Here we have our threads sending data to our main thread using a channel

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel(); // type anno is not mandatory

    let t = thread::spawn(move || {
        for i in 1..10 {
            let val = format!("{} #{}", "Ping", i);
            if let Err(e) = tx.send(val) {
                println!("Error while sending to channel: {}", e);
                break;
            }
            // Cannot print val because ownership was taken when tx.send()
            // println!("val: {}", val);
        }
    });

    for _i in 1..10 {
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    }

    t.join().unwrap();
}
