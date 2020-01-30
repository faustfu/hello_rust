use rand::prelude::*;

pub fn cr1() {
  let mut rng = rand::thread_rng();
  let x:f64 = rng.gen();

  println!("a random number is {}", x);

  for i in 1..10 {
    if rand::random() { println!("{} is true", i)}
  }
}