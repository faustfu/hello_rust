use std::fs::File;
use std::io::prelude::*;

const FILE_PATH:&str = "output.txt";

pub fn run() {
  let mut file = File::create(FILE_PATH).expect(&format!("Create {} error", FILE_PATH));

  file.write_all(b"hiii\n").expect("Write file error");
}