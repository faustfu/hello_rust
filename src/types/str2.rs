// 1. str use utf8 to encode/decode.
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

  println!("byte length of 一二三 is {}", "一二三".len());

  println!("length of 一 is {}", '一'.len_utf8());
}
