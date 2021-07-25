use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;

pub fn run() {
    let n_workers = 42;
    let n_jobs = 23;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0)); // Share an atomic variable between threads.

    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

    // create a barrier that waits for all jobs plus the starter thread
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for _ in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();

        pool.execute(move || {
            // do the heavy work
            an_atomic.fetch_add(1, Ordering::Relaxed);

            // then wait for the other threads
            barrier.wait();
        });
    }

    // wait for the threads to finish the work
    barrier.wait();

    println!("n_jobs = {}", an_atomic.load(Ordering::SeqCst));
}
