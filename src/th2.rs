// 1. Threads use channels to communicate.
// 2. "mpsc" stands for multiple producer, single consumer.

use std::sync::mpsc;
use std::thread;

pub fn th2() {
    let (tx, rx) = mpsc::channel(); // Create a simple streaming channel

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send a value to get a "Result".
    });

    let received = rx.recv().unwrap(); // Block the thread to wait for a "Result".
    println!("Got: {}", received);
}
