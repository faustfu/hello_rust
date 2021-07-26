// 1. Use crate::color_eyre to standardize third-party error reports.

use std::io;
use std::io::Read;
use std::fs::File;

use color_eyre::Report;

pub fn run() {
    match case1() {
        Ok(()) => println!("ok"),
        Err(e) => println!("new error is {}", e),
    }

    match case2() {
        Ok(()) => println!("ok"),
        Err(e) => println!("new error is {}", e),
    }

    match case3() {
        Ok(()) => println!("ok"),
        Err(e) => println!("new error is {}", e),
    }
}

fn case1() -> Result<(), Report> {
    setup()?;

    println!("Hello");

    Ok(())
}

fn case2() -> Result<(), Report> {
    let mut s = String::new();

    File::open("hello3.txt")?.read_to_string(&mut s)?; // shorten the statements

    Ok(())
}

fn case3() -> Result<(), Report> {
    panic!("PPP")
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?; // Install the panic and error report handlers:

    Ok(())
}
