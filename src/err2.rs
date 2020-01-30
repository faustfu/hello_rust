// 1. when using shortcut "?", the handled error type will be converted to the function error type automatically.
// 2. shortcut "?" could only use in functions that return Result<T, E>.
// 3. "try!" macro is the same as shortcut "?".

use std::io;
use std::io::Read;
use std::fs::File;

pub fn err2() {
  
  //case 1(combine match and Result)
  match read_username_from_file1() {
    Ok(username) => println!("username is {}", username),
    Err(e) => println!("error is {}", e),
  }

  //case 2(combine match and Result with shortcut "?")
  match read_username_from_file2() {
    Ok(username) => println!("username is {}", username),
    Err(e) => println!("error is {}", e),
  }

  //case 3(combine match and Result with chained shortcut "?")
  match read_username_from_file3() {
    Ok(username) => println!("username is {}", username),
    Err(e) => println!("error is {}", e),
  }
}

fn read_username_from_file1() -> Result<String, io::Error> { // Ok contains String and Err contains io:Error.
    let f = File::open("hello1.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // propogate the error to caller to deal with.
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // return Result => Ok or Err.
}

fn read_username_from_file2() -> Result<String, io::Error> {
  let mut f = File::open("hello2.txt")?; //propogate the error to caller if sth is wrong
  let mut s = String::new();
  f.read_to_string(&mut s)?; //propogate the error to caller if sth is wrong
  Ok(s) //pack unpacked value and return it
}

fn read_username_from_file3() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello3.txt")?.read_to_string(&mut s)?;

  Ok(s)
}
