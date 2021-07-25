// 1. Rust uses system thread by default.
// 2. The new thread will be stopped when the main thread ends.
// 3. thread::sleep will stop its execution and allow a different thread to run.
// 4. The spawned's lefttime is uncertain. Child thread may be longer than parent thread except main thread.
//    So the spawned has to copy or keep the ownership of used variables.

use std::thread;
use std::time::Duration;

pub fn run() {
    // case 1(spawn a worker thread)
    let handle1 = thread::spawn(|| { // it will spawn another thread to run the closure.
        for i in 1..10 {
            println!("Thread 1) hi number {}!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });

    // case 2(spawn a thread with moved ownership)
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Thread 2) Here's a vector: {:?}!", v);
        thread::sleep(Duration::from_secs(2));
    });

    // use sleep() to suspend the thread.
    for i in 1..5 {
        println!("Thread main) hi number {}!", i);
        thread::sleep(Duration::from_secs(1));
    }

    handle2.join().unwrap(); // wait for the spawned thread 2 be completed.
    handle1.join().unwrap(); // wait for the spawned thread 1 be completed.
}
