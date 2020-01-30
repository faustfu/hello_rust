// 1. use "const" to declare a immutable variable(binding) outside of functions.
// 2. In a file, the declaration location of functions doesn't matter.
// 3. Rust is an expression-based language. (Statement doesn't return value, ex: variable and functoin declaration...)
// 4. Expressions do not include ending semicolons.
//    If you add a semicolon to the end of an expression, you turn it into a statement.
// 5. use "return" statement in a function could turn a value.
use std::collections::HashMap;

// //normal
// const F0: u64 = 1;
// const F1: u64 = 1;
// fn fib(n: u64) -> u64 {
//   if n == 0 {
//     F0
//   } else if n == 1 {
//     F1
//   } else {
//     fib(n - 1) + fib(n - 2)
//   }
// }

//dynamic
fn fib(n:u64, map:&mut HashMap<u64, u64>) -> u64 {
  match n {
    0 | 1 => 1,
    n => {
      if map.contains_key(&n) {
        *map.get(&n).unwrap()
      } else {
        let val = fib(n - 1,map) + fib(n - 2,map);
        map.insert(n, val);
        val
      }
    }
  }
}

fn tu1() -> (i32, i32) {
  (1,2)
}

pub fn fn1() {
  let mut map = HashMap::new();

  for n in 1..40 {
    println!("fib({}) is {}", n, fib(n, &mut map));
  }

  let (i, j) = tu1();
  println!("i = {}", i);
  println!("j = {}", j);
}
