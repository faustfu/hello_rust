use std::env; //use std::env::args_os to process invalid unicode strings.
use std::process;

use hello_world;

pub fn io2() {
  let args: Vec<String> = env::args().collect();

  let config = hello_world::Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  // run() may return errors.
  if let Err(e) = hello_world::run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}
