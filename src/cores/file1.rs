use std::fs::File;
use std::io::prelude::*;

const FILE_PATH:&str = "README.txt";

pub fn run() {
  let mut file = File::open(FILE_PATH).expect(&format!("Open {} error", FILE_PATH));

  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Read the file error");

  println!("File content is\n\n{}", contents);
}