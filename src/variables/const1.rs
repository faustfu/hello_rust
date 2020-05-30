//1. conststants could be declared in any copes.
//2. declarations must includes types and a known value or constant expression.
//3. underscore in value is ignored.
const MAX : u128 = 100_000;

pub fn run() {
  const MIN : u128 = 100_000___3;
  println!("const value is {} and {}", MAX, MIN);
}