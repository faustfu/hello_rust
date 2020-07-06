// 1. Build a complex instance by a builder.
struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

struct CircleBuilder {
  x: f64,
  y: f64,
  radius: f64,
}
impl Circle {
  // create a builder for help
  fn new() -> CircleBuilder {
    CircleBuilder {
      x: 0.0,
      y: 0.0,
      radius: 1.0,
    }
  }

  // normal methods
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}
// the builder is to provide methods to set up the instance
impl CircleBuilder {
  fn x(&mut self, coordinate: f64) -> &mut Self {
    self.x = coordinate;
    self
  }
  fn y(&mut self, coordinate: f64) -> &mut Self {
    self.y = coordinate;
    self
  }
  fn radius(&mut self, radius: f64) -> &mut Self {
    self.radius = radius;
    self
  }
  fn build(&self) -> Circle {
    Circle {
      x: self.x,
      y: self.y,
      radius: self.radius,
    }
  }
}

pub fn run() {
  let c = Circle::new().x(1.0).y(2.0).radius(2.5).build();
  println!("build area is {}", c.area());
}
