use rand::prelude::*;

pub fn run() {
  let s1 = b"abc"; // convert a str to be a byte slice.

  for i in s1.iter() {
    println!("s1 = {}", i);
  }

  let mut rng = rand::thread_rng();
  let x:f64 = rng.gen();

  println!("a random number is {}", x);

  for i in 1..10 {
    if rand::random() { println!("{} is true", i)}
  }
}
