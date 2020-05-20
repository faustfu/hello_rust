// 1. A match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it,
//  and each coin falls through the first hole it encounters that it fits into.
// 2. A match expression will return a value, so every branche results have to be same type.
pub fn match1() {
  let code = 45;

  let country = match code {
    44 => "UK",
    40..=50 => "EU",
    _ => "Unknown"
  };

  println!("country is {}", country)
}