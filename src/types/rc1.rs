// 1. Use Rc<T> for uncertain reference cases.
// 2. Rc<T> is only for use in single-threaded scenarios.
// 3. Rc::clone is not a deep copy, it just increments the reference count.
// 4. Rc<T> could share data between multiple parts of program for reading only.
// 5. An Rc<T> instance is only cleaned up if its strong_count is 0.

use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>), // normal element type
  Nil,                 // end element type
}

use List::{Cons, Nil};

pub fn run() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));

  let _b = Cons(3, Rc::clone(&a)); // a is refered by b.
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = Cons(4, Rc::clone(&a)); // a is also refered by c.
    println!("count after creating c = {}", Rc::strong_count(&a));
  } // "_c" is dropped and reference count of "a" will be decreased 1 automically.
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
