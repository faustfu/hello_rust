// 1. We can use generic types in structs and enums(like option and result).

#[derive(Debug)]
struct Point1<T> {
  x: T,
  y: T,
}

impl<T> Point1<T> {
  fn x(&self) -> &T {
      &self.x
  }
}

// add a specified method for a specified type
impl Point1<f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

#[derive(Debug)]
struct Point2<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point2<T, U> {
  fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
      Point2 {
          x: self.x,
          y: other.y,
      }
  }
}

pub fn gen2() {
  // case 1(use a generic type in structs)
  let integer = Point1 { x: 5, y: 10 };
  let float = Point1 { x: 1.0, y: 4.0 };

  println!("integer struct is {:?}", integer);
  println!("float struct is {:?}", float);

  // case 2(use two generic types in structs)
  let integer_and_float = Point2 { x: 5, y: 4.0 };
  println!("integer and flost struct is {:?}", integer_and_float);

  // case 3(implement methods with generic types)
  println!("integer.x = {}", integer.x());

  // case 4(implement methods with specified type)
  println!("distance of float = {}", float.distance_from_origin());

  // case 5(implement methods with other generic types)
  let odd = Point2 { x: "Hello", y: 'c'};
  let mixed = integer_and_float.mixup(odd);
  println!("mixed.x = {}, mixed.y = {}", mixed.x, mixed.y);
}