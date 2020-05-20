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

    if y == 3 { break; }

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

  for (pos, y) in (30..41).enumerate() { // ref: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
    println!("y = {}, index = {}", y, pos)
  }
}
