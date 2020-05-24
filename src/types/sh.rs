// 1. like tuple, use struct to group relative information.
// 2. structs without field names are tuple structs.
// 3. struct could have no any fields. (utility object)
// 4. struct fields could be a reference with lifetime settings.
// 5. Methods of the struct can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably,
//    just as they can any other parameter.
// 6. Rust automatically adds in &, &mut, or * so object matches the signature of the method.
// 7. Associated Functions: functions defined in a struct, but don't need a struct instance to work.

#![allow(dead_code)]

use std::mem;

#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
  z: f64,
}

impl Point {
  fn distance(&self) -> f64 {
    let raw = self.x * self.x + self.y * self.y + self.z * self.z;
    raw.sqrt()
  }

  fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn origin() -> Point {
  Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  }
}

pub fn heap() {
  let p1 = origin();
  let p2 = Box::new(origin());
  let p3 = Point {
    x: 3.2,
    z: 5.4,
    y: 101.7,
  };
  let p4 = Point { x: 9.9, ..p3 }; //set p3 as default.
  let p5 = Point::new(1.2, 3.4, 5.6);
  let red = Color(255, 0, 0);

  println!("red color = {:#?}", red);
  println!("red color 1 = {:?}", red.0);

  println!("size of p1 = {}", mem::size_of_val(&p1));
  println!("size of p2 = {}", mem::size_of_val(&p2));
  println!("p3 = {:?}", p3);
  println!("p4's distance from (0,0,0) is {}", p4.distance());
  println!("p5 = {:?}", p5);
}
