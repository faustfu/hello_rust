// 1. Use infinite loop and blocks to access shared variables;

use std::sync::{Arc, Mutex};
use std::thread;

const RUNS: u64 = 8;
const TARGET_FLIPS: u64 = 20;

pub fn run() {
  let total_flips = Arc::new(Mutex::new(0));
  let completed = Arc::new(Mutex::new(0));

  for _ in 0..RUNS { // Spawn threads to do parallel jobs.
    let total_flips = total_flips.clone();
    let completed = completed.clone();

    thread::spawn(move || {
      flip_sim(TARGET_FLIPS, total_flips);

      let mut completed = completed.lock().unwrap();
      *completed += 1; // Use arc variables to disclose job status.
    });
  }

  loop {
    let completed = completed.lock().unwrap();
    if *completed == RUNS {
      let total_flips = total_flips.lock().unwrap();
      println!("Final average: {}", *total_flips / *completed);
      break;
    }
  }
}

fn flip_sim(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
  let mut continue_positive = 0;
  let mut iter_counts = 0;

  while continue_positive <= target_flips {
    iter_counts += 1;
    let pro_or_con = rand::random();
    if pro_or_con {
      continue_positive += 1;
    } else {
      continue_positive = 0;
    }
  }
  println!("iter_counts: {}", iter_counts);

  let mut total_flips = total_flips.lock().unwrap();
  *total_flips += iter_counts; // Use arc variables to aggregate job results.
}
