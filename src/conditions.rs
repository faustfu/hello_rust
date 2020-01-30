// IF
// 1. the type of if evaluation result(condition) has to be boolean.
// 2. if there are more than one conditions is matched, Rust will just run the first matched block.
// 3. "if" is a expression. So every blocks should return same type values.
pub fn con1() {
  let a = 2;
  let b = if a == 1 {"a == 1"} else {"a != 1"}; // 

  println!("a = {}, b = {}", a, b);
}

// LOOP
// 1. "loop" is a expression. "break" could return expression value after it.(like "return")
pub fn while1() {
  let mut x = 0;
  let mut y = 0;

  while x < 5 {
    x += 1;

    if x == 3 { continue; }

    println!("x = {}", x);
  }

  loop {
    y += 1;

    if y ==3 { break; }

    println!("y = {}", y);
  }
}

pub fn for1() {
  for x in 1..5 {
    println!("x = {}", x)
  }

  for x in (1..5).rev() { //reverse
    println!("x = {}", x)
  }

  for (pos, y) in (30..41).enumerate() {
    println!("y = {}, index = {}", y, pos)
  }
}

// 1. A match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it,
//  and each coin falls through the first hole it encounters that it fits into.
pub fn match1() {
  let code = 45;

  let country = match code {
    44 => "UK",
    40..=50 => "EU",
    _ => "Unknown"
  };

  println!("country is {}", country)
}