// 1. Result is generic enum of Ok and Err. Therefore it could be ok content or error reason.
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// 2. Use method:unwrap to parse a Ok result or panic the system.
// 3. Use question mark operator to parse a Ok result or return error.

pub fn run() {
  let i = -2;
  match h(i) {
    Ok(v) => println!("h({}) is {}", i, v),
    Err(e) => println!("{}", e),
  }
}

fn h(i: i32) -> Result<i32, String> {
  match i {
    i if i >= 0 => Ok(i + 10),
    _ => Err(format!("Input value is less then 0, found: {}", i)),
  }
}
