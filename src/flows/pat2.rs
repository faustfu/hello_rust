// 1. function use pattern match to bind arguments.
#[derive(Debug)]
struct S {
  i: i32,
}

fn f(ref s: S) {
  // parse the argument and bind its reference to an immutable variable.
  println!("{:p} = {}", s, s.i);
}

fn swap((x, y): (&str, i32)) -> (i32, &str) {
  (y, x)
}

pub fn run() {
  // case 1(use "ref"/"ref mut" in function arguments)
  let s = S { i: 42 };
  f(s);
  // println!("{:?}", s); // It is not a borrowing operation, and s is moved into the function.

  // case 2(use tuple to destructure function arguments)
  let t = ("Bob", 18);
  assert_eq!(swap(t), (18, "Bob"));
}
