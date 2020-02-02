// 1. Use "cargo test" to run test cases.
// 2. Use attribute:test before functions to indicate test cases.
// 3. Use macro:assert(assert_eq!, assert_ne!, assert!) to test expressions. Tests will call macro:panic if failed.
// 4. Macro:assert could print additional messages as tests are failed.
// 5. Use attribute:should_panic before functions to indicate the test case should panic.
// 6. Returning Result<T, E>! could replace macro:assert.
// 7. Test cases run in parallel using threads by default.
// 8. "cargo test -- --test-threads=1" will run test cases by using one thread.
// 9. "cargo test -- --nocapture" will show normal output messages.
// 10. "cargo test <keyword>" will run filtered test cases by function name.
// 11. Use attribute:ignore before function to ignore the test case.
#![allow(dead_code)]

pub fn tt1() {
  println!("tt1");
}

#[test]
fn it_works() {
  assert_eq!(4, add(2, 2));
}

#[test]
fn another() {
  assert_eq!(3, add(2, 2), "Huh?");
}

#[test]
fn larger_can_hold_smaller() {
  let larger = Rectangle {
    width: 8,
    height: 7,
  };
  let smaller = Rectangle {
    width: 5,
    height: 1,
  };

  assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
  let larger = Rectangle {
    width: 8,
    height: 7,
  };
  let smaller = Rectangle {
    width: 5,
    height: 1,
  };

  assert!(!smaller.can_hold(&larger));
}

#[test]
#[should_panic]
fn greater_than_100() {
  Guess::new(200);
}


#[test]
fn it_works_again() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

// utilities
fn add(a: i8, b: i8) -> i8 {
  a + b
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

struct Guess {
  value: i32,
}

impl Guess {
  fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }
}
