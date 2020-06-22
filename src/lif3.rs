// 1. Use lifetime annotation in reference traits.
// 2. in Rust 2018, the lifetime annotation is not necessary.
use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print_ref<'a, T>(t: &'a T)
where
  T: Debug + 'a,
{
  println!("print_ref) t is {:?}", t);
}

fn print<T>(t: T)
where
  T: Debug,
{
  println!("print) t is {:?}", t);
}

pub fn run() {
  let x = 7;
  let ref_x = Ref(&x);
  print_ref(&ref_x);
  print(ref_x); // two functions ase same.
}
