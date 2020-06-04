// 1. A trait could keep its associated types as its properties.
// 2. Associated type info are injected by Implementors.
// 3. A trait with associated types will not specify a concrete type as using like generic type.

use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add for Point { // use default types to implement the trait.
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

pub fn run() {
  println!(
    "add result is {}",
    Point { x: 1, y: 0 } + Point { x: 2, y: 3 }
  );
}
