// 1. use module:atomic to do atomic operations.
// 2. Ordering::SeqCst => "store" operations first, then "load" operations.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn run() {
  println!("atom1");
  let spinlock = Arc::new(AtomicUsize::new(1));
  let spinlock_clone = spinlock.clone();

  let thread = thread::spawn(move || {
    spinlock_clone.store(0, Ordering::SeqCst); // access the value without locks.
  });

  while spinlock.load(Ordering::SeqCst) != 0 { // access the value without locks.
     // retry if the value is not equal to zero.
  }

  if let Err(e) = thread.join() {
    println!("Thread had an error: {:?}", e);
  }
}
