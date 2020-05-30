// 1. Recoverable errors => Result<T, E>, unrecoverable errors => panic!.
// 2. Result is a enum type with two generic type parameters.
//    enum Result<T, E> {
//      Ok(T),
//      Err(E),
//    }

use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
  // // case 1(call macro:panic directly)
  // panic!("Huh?");

  // // case 2(cause by illegal operations)
  // let v = vec![1, 2, 3];
  // v[99];

  // case 3(return Result)
  let f = File::open("hello.txt"); // return type is Result
  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      // there are many kinds
      ErrorKind::NotFound => match File::create("hello.txt") {
        // return type is Result
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => panic!("Problem opening the file: {:?}", other_error),
    },
  }; // f's type is File.
  println!("file handler is {:?}", f);

  // case 4(return Result and deal with closures)
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| { // "if" is a expression and it will return a new file handler if no errors.
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
  println!("file handler is {:?}", f);

  // case 5(return value of Result or panic)
  let f = File::open("hello.txt").unwrap();
  println!("file handler is {:?}", f);

  // case 5(return value of Result or panic with a customized message)
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
  println!("file handler is {:?}", f);
}
