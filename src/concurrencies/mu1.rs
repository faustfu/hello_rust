// 1. Rust could use mutex to manage data sharing between blocks.

use std::sync::Mutex;

pub fn run() {
    let m = Mutex::new(5);

    println!("m = {:?}", m);

    {
        let mut num = m.lock().unwrap(); // block the process to get the lock or an error.
        *num = 6; // reset the value and then unlock as closing the scope.
    }

    println!("m = {:?}", m);
}
