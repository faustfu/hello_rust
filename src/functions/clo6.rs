// use iterators directly to skip clone()
use std::env;
use std::process;

#[derive(Debug)]
struct Config {
  query: String,
  filename: String,
  case_sensitive: bool,
}

impl Config {
  fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn clo6() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("config is {:?}", config);
}
