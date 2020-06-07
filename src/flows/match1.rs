// 1. A match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it,
//  and each coin falls through the first hole it encounters that it fits into.
// 2. A match expression will return a value, so every branche results have to be same type.
// 3. Question mark is a Result matching shortcut. It will parse the result or return error immediately.
pub fn run() {
  // case 1
  let code = 45;

  let country = match code {
    44 => "UK",
    40..=50 => "EU",
    _ => "Unknown",
  };

  println!("country is {}", country);

  // case 2
  match question(Ok(1)) {
    Ok(i) => println!("new value is {}", i),
    Err(e) => println!("new error is {}", e),
  }
}

fn question(i: Result<i32, &str>) -> Result<i32, &str>  {
  let value = i?;

  Ok(value + 10)
}
