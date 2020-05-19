// 1. Rust uses system thread by default.
// 2. The new thread will be stopped when the main thread ends.
// 3. thread::sleep will stop its executio and allow a different thread to run.
// 4. The spawned is lefttime is uncertain. So the spawned has to keep the ownership of used variables.

use std::thread;
use std::time::Duration;

pub fn th1() {
    let handle1 = thread::spawn(|| { // it will spawn another thread to run the closure.
        for i in 1..10 {
            println!("Thread 1) hi number {}!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Thread 2) Here's a vector: {:?}!", v);
        thread::sleep(Duration::from_millis(1));
    });

    for i in 1..5 {
        println!("Thread main) hi number {}!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle2.join().unwrap(); // wait for the spawned thread 2 be completed.
    handle1.join().unwrap(); // wait for the spawned thread 1 be completed.
}
