use std::env; //use std::env::args_os to process invalid unicode strings.
use std::error::Error;
use std::fs;
use std::process;

pub fn io2() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  run(config).unwrap_or_else(|err| {
    println!("Run error: {}", err);
    process::exit(1);
  });
}

// Encapsulate main process logic and map return uncertain errors.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  Ok(())
}

// Use struct to group relative variables.
struct Config {
  query: String,
  filename: String,
}

// Encapsulate the initial logic of members into a method likes OOP.
impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}
