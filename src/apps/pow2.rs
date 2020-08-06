// 1. Use thread pools to do "Proof of Work".

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use itertools::Itertools;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;

const BASE: usize = 42;
const THREADS: usize = 8;
static DIFFICULTY: &'static str = "00000";
struct Solution(usize, String);

pub fn run() {
  println!(
    "PoW: find a number, SHA256(the number * {}) == \"{}...\" ",
    BASE, DIFFICULTY
  );
  let pool = ThreadPool::new(num_cpus::get());
  println!("Started pool...");
  println!("Please wait...");

  let is_found = Arc::new(AtomicBool::new(false));
  let (sender, receiver) = mpsc::channel();

  for i in 0..THREADS {
    let sender_n = sender.clone();
    let is_found = is_found.clone();
    pool.execute(move || {
      find(i, sender_n, is_found);
    });
  }

  match receiver.recv() {
    Ok(Solution(i, hash)) => {
      println!("Found the solution: ");
      println!("The number is {}, and hash is {}.", i, hash);
    }
    Err(_) => panic!("Worker threads disconnected!"),
  }
}

// verify if the number is matched the rules.
fn verify(number: usize) -> Option<Solution> {
  let mut hasher = Sha256::new();
  hasher.input_str(&(number * BASE).to_string());

  let hash: String = hasher.result_str();

  if hash.starts_with(DIFFICULTY) {
    Some(Solution(number, hash))
  } else {
    None
  }
}

fn find(start_at: usize, sender: mpsc::Sender<Solution>, is_found: Arc<AtomicBool>) {
  for number in (start_at..).step_by(THREADS) {
    if is_found.load(Ordering::Relaxed) {
      return;
    }

    if let Some(solution) = verify(number) {
      is_found.store(true, Ordering::Relaxed);
      sender.send(solution).unwrap();
      return;
    }
  }
}
