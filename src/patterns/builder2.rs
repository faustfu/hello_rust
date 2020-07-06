//
use std::process::Command;

pub fn run() {
  Command::new("ls")
    .arg("-l")
    .arg("-a")
    .spawn()
    .expect("ls command failed to start");
}
