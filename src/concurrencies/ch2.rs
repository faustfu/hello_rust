// 1. Clone transmitters to get multiple producers. 

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx); // tx and tx1 will transmit values to rx.
    thread::spawn(move || {
        let vals = vec![
            String::from("1)hi"),
            String::from("1)from"),
            String::from("1)the"),
            String::from("1)thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    }); // tx1 is closed in the spawned thread.

    thread::spawn(move || {
        let vals = vec![
            String::from("2)more"),
            String::from("2)messages"),
            String::from("2)for"),
            String::from("2)you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }); // tx is closed in the spawned thread.

    for received in rx {
        // Use for loop to block the flow and receive values.
        println!("Got: {}", received);
    }
}
