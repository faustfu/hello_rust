// If the value is immutable borrow, It could not be transfered(mutated) until the borrowing process is ended.
// Immutable borrowing => freeze and clone the original value.

struct Foobar(u8);

fn foo(a:u8) -> Foobar {
  Foobar(a)
}

fn bar(b:Foobar) {
  println!("b is {}", b.0);
}

pub fn own2() {
  let a = foo(1);

  let b = &a; //1. borrow of `a` occurs here

  bar(a); //Cause 1&2, cannot move out of `a` because it is borrowed

  println!("b is {}", b.0); //2. borrow of `a` is accessed here

}