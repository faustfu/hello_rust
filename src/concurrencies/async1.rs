// 1. Use "async" before a block/function/closure will create an instance of futures.
// 2. A future instance has to be pulled by executor or "await".
// 3. Use default futures::executor or tokio::executor by function modifier #[tokio::main] at main function to pull future results.

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

async fn my_async_2_1() {
    println!("Hello 1");
    my_async_2_2().await
}

async fn my_async_2_2() {
    println!("Hello 2");
}

async fn my_async_2() {
    my_async_2_1().await;
    println!("Hello 3")
}

async fn my_async_3() {
    let fut_values = async {
        my_async_1_1().await;
        my_async_1_2().await
    };

    fut_values.await;
    println!("Hello 3")
}

pub fn run() {
    println!("async1");
    executor::block_on(my_async_1()); // Use executor to block and pull future result.

    println!("async2");
    executor::block_on(my_async_2()); // Use executor to block and pull future result.

    println!("async3");
    executor::block_on(my_async_3()); // Use executor to block and pull future result.
}