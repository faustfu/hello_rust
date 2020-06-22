// 1. Functions are first class citizen in Rust.
// 2. A function could be transfered with "fn" and its signature.
// 3. Functions could not access context like closures.

fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
  op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn product(a: i32, b: i32) -> i32 {
  a * b
}

type MathOp = fn(i32, i32) -> i32;
fn calc(op: MathOp, a: i32, b: i32) -> i32 {
  println!("{:p}", op);
  op(a, b)
}
fn process(op: &str) -> MathOp {
  fn sub(a: i32, b: i32) -> i32 {
    a - b
  }
  fn div(a: i32, b: i32) -> i32 {
    a / b
  }

  match op {
    "sub" => sub,
    "div" => div,
    _ => {
      panic!("Unknown op {}!", op)
    }
  }
}

pub fn run() {
  // case 1(use fn directly)
  let (a, b) = (2, 3);
  println!("a + b = {}", math(sum, a, b));
  println!("a * b = {}", math(product, a, b));

  // case 2(use type to wrap function signature)
  let (c, d) = (8, 2);
  println!("c + d = {}", calc(sum, c, d));
  println!("c * d = {}", calc(product, c, d));
  println!("c - d = {}", process("sub")(c,d));
  println!("c / d = {}", process("div")(c,d));
}
