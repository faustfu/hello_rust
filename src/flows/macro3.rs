// 1. Prodedural macros could change token stream actively.

use hello_proc_macro::A;

pub fn run() {
  println!("macro3");
}

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
  assert_eq!("hello from impl A".to_string(), A.a());
}
