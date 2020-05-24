#[derive(Debug)]
// Ownership is a way of using stack to replace garbage collection or record count to manage allocated memory space.

// Ownership starts off with the following rules:
// 1. Each value in Rust has a variable that’s called its owner.(binding)
// 2. There can only be one owner at a time.
// 3. When the owner goes out of its scope, the binded value will be dropped if it is not a return value(move).
// 4. A scope is the range within a program for which an item is valid. 
// 5. Borrow: get the reference of the owner for accessing the actual value.
// 6. If the borrowing is readonly, it will not do any data races.
// 7. The lifetime of the value always outlives the references to it.
// 8. If the ownership is transfered. New owner could decide the value's mutation state.
// 9. A value could be clone(unknown size in heap)/copy(known size in stack) to avoid ownership transfering(move) issue. But it has cost.

struct Foobar(i32);

impl Drop for Foobar {
  fn drop(&mut self) {
    println!("Dropping a Foobar: {:?}", self);
  }
}

fn uses_foobar(foobar: Foobar) {
  println!("I consumed a Foobar: {:?}", foobar);
}

pub fn own1() {
  let x = Foobar(1);
  uses_foobar(x); //value is moved/transfered here
  // uses_foobar(x); //the statement is invalid, because the value is used here after move

  println!("Before x");
  let _x = Foobar(1);
  {
    println!("Before y");
    let _y = Foobar(2);
    println!("After y");
  } //end of _y
  println!("After x");
  //end of _x
}
