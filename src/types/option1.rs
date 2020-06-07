// 1. Option is generic enum of Some and None. Therefore it could be some value or none.
// enum Option<T> {
//     Some(T),
//     None,
// }
// 2. Choosing between match and if let depends on what you’re doing in your particular situation and
//     whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
/* map signature
impl Option<T>{
  fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U { ... }
}
*/

pub fn run() {
  // case 1(match)
  let x = 3.0;
  let y = 2.0;
  let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

  println!("result = {:?}", result);

  match result {
    Some(z) => println!("{} / {} = {}", x, y, z),
    None => println!("{} / {} has errors", x, y),
  }

  // case 2(if let)
  // parsing is ok, then "if" block will be executed.
  if let Some(z) = result {
    println!("z = {}", z)
  };

  // case 3(map with functions)
  let i = Some(5);
  if let Some(z) = maybe_add_4(i) {
    println!("z = {}", z)
  };

  // case 4(map with closures)
  let i = Some(5);
  if let Some(z) = maybe_add_5(i) {
    println!("z = {}", z)
  };
}

fn maybe_add_4(i: Option<i32>) -> Option<i32> {
  i.map(add_4)
}

fn add_4(i: i32) -> i32 {
  i + 4
}

fn maybe_add_5(i: Option<i32>) -> Option<i32> {
  i.map(|x| x + 5)
}
