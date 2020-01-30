// 1. use "cargo test" to run test cases.
// 2. use attribute:test before functions to indicate test cases.
// 3. use macro:assert_eq to test expressions.
pub fn tt1() {
  println!("tt1");
}

#[test]
fn it_works() {
  assert_eq!(4, add(2, 2));
}

#[test]
fn another() {
  assert_eq!(3, add(2, 2));
}

fn add(a:i8, b:i8) -> i8 {
  a + b
}

