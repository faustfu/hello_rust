// 1. Patterns are used to match values against structures and to, optionally, bind variables to values inside these structures.
// 2. Patterns can be used to destructure structs, enums, and tuples.
struct Person {
  car: Option<bool>,
  age: u8,
  name: String,
  size: u8,
}

enum Message {
  Quit,
  WriteString(String),
  Move { x: u8, y: u8 },
  ChangeColor(u8, u8, u8),
}

struct RGBA {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

const TROPOSPHERE_MIN: u64 = 0;
const TROPOSPHERE_MAX: u64 = 9999;
const STRATOSPHERE_MIN: u64 = 10000;
const STRATOSPHERE_MAX: u64 = 99999;
const MESOSPHERE_MIN: u64 = 100000;
const MESOSPHERE_MAX: u64 = 999999;

pub fn run() {
  // case 1(if let)
  let person = Person {
    car: Some(true),
    age: 16,
    name: String::from("abc"),
    size: 29,
  };

  // All three conditions have to be passed to run inner block statements.
  if let Person {
    car: Some(_),
    age: person_age @ 13..=19, // Use "@" to bind variables for later usage.
    name: ref person_name, // Use "ref" to enforce binding a variable by reference.
    .. // Use wildcard ".." to ignore other properties.
  } = person
  {
    println!("{} has a car and is {} years old.", person_name, person_age);
  }

  // case 2(match)
  let message = Message::ChangeColor(22, 55, 88);

  match message {
    Message::Quit => println!("Quit"),
    Message::WriteString(write) => println!("{}", &write),
    Message::Move { x, y: 0 } => println!("move {} horizontally", x),
    Message::Move { .. } => println!("other move"),
    Message::ChangeColor {
      0: red,
      1: green,
      2: _, // Use wildcard "_" to ignore a value.
    } => {
      println!("color change, red: {}, green: {}", red, green);
    }
  };

  // case 3(refutable)
  if let (a, 3) = (1, 2) {
    // "(a, 3)" is refutable, and will not match
    panic!("Shouldn't reach here, a = {}", a);
  } else if let (a, 4) = (3, 4) {
    // "(a, 4)" is refutable, and will match
    println!("Matched ({}, 4)", a);
  }

  // case 4(literal patterns)
  for i in -2..5 {
    match i {
      -1 => println!("It's minus one"),
      1 => println!("It's a one"),
      2 | 4 => println!("It's either a two or a four"),
      _ => println!("Matched none of the arms"),
    }
  }

  // case 5(identifier patterns)
  let mut variable = 10; // Bind/declare a variable with the value.
  variable += 5;
  println!("New variable is {}", variable);

  let x = 2;
  match x {
    e @ 1..=5 => println!("got a range element {}", e), // Bind a variable with the value matched in this arm.
    _ => println!("anything"),
  }

  // Binding will copy or move a value by default. But it could be change to binding to a reference
  let y = &x;
  println!("y is {}", y);

  let i = 6;
  let z = &Some(i);
  match z {
    None => (),
    // Rust will detect the reference and v will be unboxed and boxed to be a reference variable again.
    Some(v) => println!("z is {} inside", v),
  }
  println!("z is {} outside", z.unwrap());

  // case 6(wildcard pattern)
  let (a, _) = (10, x); // the x is always matched by _
  println!("a is {}", a);

  // ignore a function/closure param
  let real_part = |a: f64, _: f64| a;
  println!("real_part is {}", real_part(5.6, 6.7));

  // ignore a field from a struct
  let color = RGBA {
    r: 22,
    g: 44,
    b: 66,
    a: 88,
  };
  let RGBA {
    r: red,
    g: _,
    b: blue,
    a: _,
  } = color;
  println!("red is {}, blue is {}", red, blue);

  // accept any Some, with any value
  let x = Some(red);
  if let Some(_) = x {
    println!("x is something");
  }

  // case 7(range pattern)
  let c = 's';
  let valid_variable = match c {
    'a'..='z' => true,
    'A'..='Z' => true,
    'α'..='ω' => true,
    _ => false,
  };
  println!("c is a valid character? {}", valid_variable);

  let ph = 13;
  println!(
    "The liquid is {}",
    match ph {
      0..=6 => "acid",
      7 => "neutral",
      8..=14 => "base",
      _ => unreachable!(),
    }
  );

  // using paths to constants:
  let altitude = 34567;
  println!(
    "the altitude is in {}",
    match altitude {
      TROPOSPHERE_MIN..=TROPOSPHERE_MAX => "troposphere",
      STRATOSPHERE_MIN..=STRATOSPHERE_MAX => "stratosphere",
      MESOSPHERE_MIN..=MESOSPHERE_MAX => "mesosphere",
      _ => "outer space, maybe",
    }
  );

  let altitude_101 = 500;
  let n_times = 10;
  if let altitude @ TROPOSPHERE_MIN..=TROPOSPHERE_MAX = n_times * altitude_101 {
    println!(
      "{} times of 101 altitude is {} in troposphere",
      n_times, altitude
    );
  }

  // case 8(reference pattern)
  let int_reference = &0;

  let a = match *int_reference {
    0 => "zero",
    _ => "some",
  };
  let b = match int_reference {
    &0 => "zero",
    _ => "some",
  };
  let c = match int_reference {
    0 => "zero", // Rust could unbox the reference value automatically.
    _ => "some",
  };
  println!("a is {}, b is {}, c is {}", a, b, c);

  // case 9(grouped pattern)
  let int_reference = &3;
  match int_reference {
    &(0..=5) => println!("in 0 and 5"), // "&0..=5" or "&0..=&5" are not allowed.
    _ => println!("not in 0 and 5"),
  }

  // case 10(slice pattern)
  // Fixed size
  let arr = [1, 2, 3];
  match arr {
    [1, _, _] => println!("starts with one"),
    [a, b, c] => println!("[{}, {}, {}]", a, b, c),
  };

  // Dynamic size
  let v = vec![1, 2, 3];
  match v[..] {
    [a, b] => println!("[{}, {}]", a, b), // this arm will not apply because the length doesn't match
    [a, b, c] => println!("[{}, {}, {}]", a, b, c), // this arm will apply
    _ => println!("unknown slice"),
  };
}
