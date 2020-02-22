// 1. Rust uses system thread by default.
// 2. The new thread will be stopped when the main thread ends.
// 3. thread::sleep will stop its executio and allow a different thread to run.

use std::thread;
use std::time::Duration;

pub fn th1() {
    let handle = thread::spawn(|| { // it will spawn another thread to run the closure.
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // wait for the spawned thread be completed.
}
