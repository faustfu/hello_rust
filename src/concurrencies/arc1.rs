// 1. Use Atomic Reference Counting(arc) and Mutex to pass mutable values between threads.

use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    let counter = Arc::new(Mutex::new(0)); // counter is immutable, but mutex is not.
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone the counter reference for every handles.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // lock the mutex before accessing.

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // wait for every handles to do their jobs.
    }

    println!("Result: {}", *counter.lock().unwrap()); // lock the mutex before accessing.
}
