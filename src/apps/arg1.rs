use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod err;
mod opt;
mod read;
mod write;

use self::err::Error;
use self::opt::Opt;
use self::read::{load_csv, write_csv};
use self::write::replace_column;

pub fn run() {
  let opt = Opt::from_args();

  let filename = PathBuf::from(opt.input);
  let csv_data = match load_csv(filename) {
    Ok(fname) => fname,
    Err(e) => {
      println!("main error as reading: {:?}", e);
      process::exit(1);
    }
  };

  let modified_data = match replace_column(csv_data, &opt.column_name, &opt.replacement) {
    Ok(data) => data,
    Err(e) => {
      println!("main error as replacing: {:?}", e);
      process::exit(1);
    }
  };

  let output_file = &opt.output.unwrap_or("output/output.csv".to_string());
  match write_csv(&modified_data, &output_file) {
    Ok(_) => println!("write success!"),
    Err(e) => {
      println!("main error as writing: {:?}", e);
      process::exit(1);
    }
  };
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_valid_load_csv() {
    let filename = PathBuf::from("input/challenge.csv");
    let csv_data = load_csv(filename);
    assert!(csv_data.is_ok());
  }
}