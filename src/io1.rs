extern crate rand;

use rand::random;
use std::io;

fn get_guess() -> u8 {
  loop {
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Could not read from stdio");

    match guess.trim().parse::<u8>() { //parse() -> Result
      Ok(v) => return v, //parse successfully and return the value
      Err(e) => println!("Wrong type = {}", e), //parse failed and do next loop
    };
  }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
  if guess > correct {
    println!("lower");
    false
  } else if guess < correct {
    println!("higher");
    false
  } else {
    println!("Correct!");
    true
  }
}

pub fn run() {
  let rng: u8 = random();

  loop {
    println!("Input:");

    let guess = get_guess();

    println!("guess is {}", guess);

    if handle_guess(guess, rng) {
      break;
    }
  }
}
