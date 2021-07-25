// 1. Use sync channels.

use std::sync::mpsc::sync_channel;
use std::thread;

pub fn run() {
  let (tx, rx) = sync_channel(1);

  tx.send(1).unwrap(); // send one object would not block the channel.

  thread::spawn(move || {
    println!("in thread");

    tx.send(2).unwrap(); // send one object and block the channel.
  });

  for received in rx {
    // Use for loop to block the flow and receive values.
    println!("Got: {}", received);
  }
}
