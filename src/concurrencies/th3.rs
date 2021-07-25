// Use crate:threadpool for thread pooling.

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn run() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for i in 0..n_jobs {
        let tx = tx.clone();

        pool.execute(move || { // Use fn:execute to run a block of codes in parallel.
            tx.send(i)
                .expect("channel will be there waiting for the pool");
        });
    }

    println!("final result is {}", rx.iter().take(n_jobs).fold(0, |a, b| a + b)); // Get all numbers and reduce to be a number.
}
