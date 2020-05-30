// Combine Rc<T> with RefCell<T> for multiple mutable references.

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>), // head is mutable
  Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

pub fn run() {
  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  println!("a before = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}
