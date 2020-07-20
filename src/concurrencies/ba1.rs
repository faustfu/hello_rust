// 1. use Barrier to sync threads.

use std::sync::{Arc, Barrier};
use std::thread;

const RUNS: usize = 5;

pub fn run() {
  println!("ba1");

  let mut handles = Vec::with_capacity(RUNS);
  let barrier = Arc::new(Barrier::new(RUNS));

  for _ in 0..RUNS {
    let c = barrier.clone();
    handles.push(thread::spawn(move || {
      println!("before wait");
      c.wait(); // block 4 threads to sync all threads.
      println!("after wait");
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }
}
