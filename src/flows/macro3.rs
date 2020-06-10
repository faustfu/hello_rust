// 1. Prodedural macros could change token stream actively.

use hello_proc_macro::A;
use hello_proc_macro::attr_with_args;

pub fn run() {
  println!("macro3");
}

// case 1(use derive to implement struct:A)
#[derive(A)]
struct A;

// case 2(use custom attribute to replace implementation of fn:foo)
#[attr_with_args("Hello, Rust!")]
fn foo() {}

#[test]
fn test_derive_a() {
  assert_eq!("hello from impl A".to_string(), A.a());
}

#[test]
fn test_foo() {
  assert_eq!(foo(), "Hello, Rust!");
}