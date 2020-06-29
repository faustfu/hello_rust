// 1. Closures could access context.
// 2. Use "Box" to define a closure signature.
// 2. Use "impl Fn" to define a closure signature by trait Fn in Rust 2018.

fn counter1(i: i32) -> Box<dyn Fn(i32) -> i32> {
  Box::new(move |n: i32| n + i)
}
fn counter2(i: i32) -> impl Fn(i32) -> i32 {
  move |n: i32| n + i
  // "i" would be captured by reference by default.
  // But the closure is the return result, and it's ownership has to be transfered between functions,
  // so use "move" to move the ownership by force.
}

pub fn run() {
  // case 1
  let f1 = counter1(2);
  let f2 = counter2(3);
  println!("2 + 1 = {}", f1(1));
  println!("3 + 1 = {}", f2(1));
}
