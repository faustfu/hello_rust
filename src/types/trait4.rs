// 1. Traits could have dependency.
// 2. Supertraits has to be implemented first.
/* Three function trait ideas(not real)
//
trait FnOnce {
  type Output;
  fn call_once(self) -> Self::Output;
}
//
trait FnMut: FnOnce {
  fn call_mut(&mut self) -> Self::Output;
}
//
trait Fn: FnMut {
  fn call(&self) -> Self::Output;
}
*/

use std::fmt;

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
  }
}

struct Point {
  x: i32,
  y: i32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
  }
}

impl OutlinePrint for Point {}

pub fn run() {
  let p = Point{x:3, y:4};

  p.outline_print();
}