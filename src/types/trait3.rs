// 1. A trait is able to be declared with generic types and implemented with concrete types.

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // use concrete types to implement the trait.
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn run() {
  println!("trait3");
}