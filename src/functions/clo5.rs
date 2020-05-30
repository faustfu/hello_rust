// 1. Create a iterator by implementing next() method.

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    // return Some or None
    if self.count < 5 {
      self.count += 1;

      Some(self.count)
    } else {
      None
    }
  }
}

pub fn run() {
  // case 1(use next() to loop)
  for c in Counter::new() {
    println!("item in counter is {}", c);
  }

  // case 2(use relative methods)
  println!(
    "complex operation result is {}",
    Counter::new()
      .zip(Counter::new().skip(1))
      .map(|(a, b)| a * b)
      .filter(|x| x % 3 == 0)
      .sum::<u32>()
  );
}
