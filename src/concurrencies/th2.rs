// 1. Default size of a spawned thread is 2MB. Use std::thread::Builder could customize spawned threads.

use std::panic;
use std::thread::{current, Builder};

pub fn run() {
  let mut v = vec![];
  for id in 0..5 {
    let thread_name = format!("child-{}", id);
    let size: usize = 3 * 1024 * 1024;
    let builder = Builder::new().name(thread_name).stack_size(size);
    let child = builder
      .spawn(move || {
        let handler = current();
        let name = handler.name().unwrap();
        println!("in child: {}[{}]", name, id);

        match panic::catch_unwind(move || {
          if id == 2 {
            panic!("Failed({})", id);
          }
        }) {
          Ok(_) => println!("{} done", name),
          Err(_) => println!("do sth after failed in {}", name),
        }
      })
      .unwrap();
    v.push(child);
  }

  for child in v {
    child.join().unwrap();
  }
}
