// In a particular scope, there is only one mutable borrow is allowed.
// If the value is mutable borrowed, it cannot be accessed unitl the borrowing process is ended.
// Mutable borrowing => transfer ownership temporarily.

struct Foobar(u8);

fn foo(a:u8) -> Foobar {
  Foobar(a)
}

pub fn own3() {
  let mut a = foo(1);

  let b = &mut a; // mutable borrow(b) occurs here and continues until the block is ended.

  b.0 += 1;
  // a.0 += 1; // cannot use `a.0` because it was mutably borrowed
  // println!("a is {}", a.0); // cannot borrow `a.0` as immutable because it is also borrowed as mutable

  println!("b is {}", b.0);

  // let c = &mut a; // cannot borrow `a` as mutable more than once at a time
  // let d = &a; // cannot borrow `a` as immutable because it is also borrowed as mutable

}