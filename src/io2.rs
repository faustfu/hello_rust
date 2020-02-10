use std::env; //use std::env::args_os to process invalid unicode strings.
use std::fs;

pub fn io2() {
  let args: Vec<String> = env::args().collect();

  let (query, filename) = parse_config(&args);

  println!("Searching for {}", query);
  println!("In file {}", filename);
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);
}

// The pure function holds the logic that determines which argument goes in which variable and passes the values back to main.
fn parse_config(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let filename = &args[2];

  (query, filename)
}