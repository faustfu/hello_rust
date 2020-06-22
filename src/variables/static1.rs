// 1. Static variables are global in the module like constants, but they could be mutable in unsafe block.
static A: i32 = 1;

pub fn run() {
  println!("A is {}", A);
}
