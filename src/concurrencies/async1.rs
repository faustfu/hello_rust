// 1. Use "async" before a block will create an instance of futures.
// 2. Use "async" before a function/closure will create a generator of futures.
// 3. A future instance has to be executed by executor or "await".
// 4. A future instance can only be executed one time.

use futures::{join, executor};

async fn my_async_1_1() {
    println!("Hello 1");
}

async fn my_async_1_2() {
    println!("Hello 2");
}

async fn my_async_1() {
    let f1 = my_async_1_1();
    let f2 = my_async_1_2();

    join!(f1, f2);
}

pub fn run() {
  executor::block_on(my_async_1());
  
  println!("async1");
}