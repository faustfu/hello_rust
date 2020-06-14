// 1. Use a wrapper struct(newtype) to bypass the orphan rule and implement new methods for the type.
// 2. The inner type has to be accessed indirectly, or implement trait:Deref to expose it.

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", ")) // use self.0 to access the wrapped type.
  }
}

pub fn run() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
