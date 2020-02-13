use std::error::Error;
use std::fs;

// Use struct to group relative variables.
pub struct Config {
    pub query: String,
    pub filename: String,
}

// Encapsulate initial logic of members into a method likes OOP.
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Encapsulate main process logic and map return uncertain errors.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
