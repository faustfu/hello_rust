use std::env; //use std::env::args_os to process invalid unicode strings.
use std::fs;

pub fn io2() {
  let args: Vec<String> = env::args().collect();

  let config = parse_config(&args);

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  let contents = fs::read_to_string(config.filename)
      .expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);
}

// Use struct to group relative variables.
struct Config {
  query: String,
  filename: String,
}

// The function holds the logic that determines which argument goes in which variable and passes the values back to main.
fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let filename = args[2].clone();

  Config { query, filename }
}
