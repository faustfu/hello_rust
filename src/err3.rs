// 1. If panic!, there is no way to recover like golang. It will terminate the running process.
// 2. Returning Result is a good default choice.
// 3. The unwrap and expect methods are suitable when prototyping and refactoring later.
// 4. Test cases could take panic! as a failure easily.
// 5. Expected failures => return Result; Unexpected failures => panic!
// 6. When the input contract(interface) is invalid => panic!

pub fn err3() {
  let normal_guess = Guess::new(20);
  println!("normal guess is {}", normal_guess.value());

  let abnormal_guess = Guess::new(120);
  println!("abnormal guess is {}", abnormal_guess.value());
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}
