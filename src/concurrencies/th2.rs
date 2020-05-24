// 1. Threads use channels to communicate.
// 2. "mpsc" stands for multiple producer, single consumer.
// 3. Status of the channel has to be certain(no more input), then it could be closed.

use std::thread;
use std::sync::mpsc;

pub fn th2() {
    let (tx, rx) = mpsc::channel(); // Create a simple streaming channel, tx is the channel's input, rx is the channel's output.

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send a value and get an operation result to check if it is failed.
    });

    let received = rx.recv().unwrap(); // Block the thread to read from the channel.
    println!("Got: {}", received);
}
