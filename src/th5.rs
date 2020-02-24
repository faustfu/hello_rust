// 1. Use Atomic Reference Counting(arc) to pass values between threads.

use std::sync::{Mutex, Arc};
use std::thread;

pub fn th5() {
    let counter = Arc::new(Mutex::new(0)); // counter is immutable, but mutex is not.
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
