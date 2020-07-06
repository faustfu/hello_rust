// 1. use crate:failure to manage errors.
use failure::{Backtrace, Context, Fail};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Error {
  inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
  #[fail(display = "IoError")] //implement trait:Display by Fail macro.
  Io(#[cause] std::io::Error), //use macro:cause to implement the error.
  #[fail(display = "ParseError")]
  Parse(#[cause] std::num::ParseIntError),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.inner, f)
  }
}

impl Fail for Error {
  fn cause(&self) -> Option<&dyn Fail> {
    self.inner.cause()
  }
  fn backtrace(&self) -> Option<&Backtrace> {
    self.inner.backtrace()
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error {
      inner: Context::new(ErrorKind::Io(err)),
    }
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Error {
    Error {
      inner: Context::new(ErrorKind::Parse(err)),
    }
  }
}

type ParseResult<T> = Result<T, Error>;

fn sum(filename: &str) -> ParseResult<i32> { // wrap all kinds of errors
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let mut sum = 0;
  for c in contents.lines() {
    let n: i32 = c.parse::<i32>()?;
    sum += n;
  }

  Ok(sum)
}

pub fn run() {
  println!("fail1");
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  match sum(filename) {
    Ok(n) => println!("sum is {}", n),
    Err(e) => println!("err is {:?}", e),
  }
}
