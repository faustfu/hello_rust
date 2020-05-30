// 1. The variants of the enum are namespaced under its identifier.
// 2. Each variant's inner types could be same or different and their outer types are same => the identifier.
// 3. Enum could define methods like struct.

#[derive(Debug)]
enum Color {
  Red,
  Green,
  Blue,
  RgbColor(u8, u8, u8), //tuple
  Cmyk {
    cyan: u8,
    magenta: u8,
    yellow: u8,
    black: u8,
  }, //struct
}

#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String),
}
// add a method for the enum.
impl IpAddr {
  fn connect(&self) {
    println!("connect to {:?}", self);
  }
}

pub fn run() {
  // let c:Color = Color::RgbColor(0,0,0);
  let c: Color = Color::Cmyk {
    cyan: 1,
    magenta: 2,
    yellow: 3,
    black: 255,
  };

  let blue = Color::Blue;
  println!("Blue color is {:?}", blue);

  let home = IpAddr::V4(String::from("127.0.0.1"));
  home.connect();

  let loopback = IpAddr::V6(String::from("::1"));
  loopback.connect();

  match c {
    Color::Red => println!("Color is Red"),
    Color::Green => println!("Color is Green"),
    Color::Blue => println!("Color is Blue"),
    Color::RgbColor(0, 0, 0) => println!("Color is Black"),
    Color::RgbColor(r, g, b) => println!("Color is RgbColor({},{},{})", r, g, b),
    Color::Cmyk {
      cyan,
      magenta,
      yellow,
      black: 255,
    } => println!("Color is Cmyk({},{},{},255)", cyan, magenta, yellow),
    _ => (),
  }
}
