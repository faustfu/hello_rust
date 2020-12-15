// 1. Use crate:futures to use async.
// 2. Async block has to be executed, ex: executor, await.

use futures::{self, executor};

async fn my_async_1_1() {
    println!("Hello 1");
}

async fn my_async_1_2() {
    println!("Hello 2");
}

async fn my_async_1() {
    my_async_1_1().await;
    my_async_1_2().await;
}

pub fn run() {
  executor::block_on(my_async_1());
  println!("async1");
}