// 1. use RwLock to seperate reader and writer threads.

use std::sync::RwLock;

pub fn run() {
  println!("th8");

  let lock = RwLock::new(5);
  {
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap(); // It is ok to have multiple read locks.
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
  }

  {
    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);
  }
}
