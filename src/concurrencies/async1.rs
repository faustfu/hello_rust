// 1. Use "async" before a block will create an instance of futures.(one result)
// 2. Use "async" before a function/closure will create a generator of futures.(many results)
// 3. A future instance has to be pulled by executor or "await".
// 4. A future instance can only be executed one time.
// 5. Use default futures::executor or tokio::executor by function modifier #[tokio::main] at main function to pull future results.

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

    println!("Hello 3"); // 3 will be printed first, cause async blocks are waiting to be pulled.

    join!(f2, f1); // Use macro:join to block and pull future results at the same time.
}

pub fn run() {
  executor::block_on(my_async_1()); // Use executor to block and pull future result.
  
  println!("async1");
}