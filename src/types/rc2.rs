// 1. Use RefCell<T> to implement the interior mutability pattern.
// 2. RefCell<T> manages borrowing rules at runtime, not at compile time.
// 3. RefCell<T> is only for use in single-threaded scenarios.
// 4. In the below case, MockMessenger is immutable for LimitTracker, but it has to be mutable for mocking.

trait Messenger {
  fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T, // Messenger is injected into LimitTracker.
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
      LimitTracker {
          messenger,
          value: 0,
          max,
      }
  }

  pub fn set_value(&mut self, value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
          self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
          self.messenger
              .send("Urgent warning: You've used up over 90% of your quota!");
      } else if percentage_of_max >= 0.75 {
          self.messenger
              .send("Warning: You've used up over 75% of your quota!");
      }
  }
}

struct MockMessenger {
  sent_messages: Vec<String>,
}

impl MockMessenger {
  fn new() -> MockMessenger {
      MockMessenger {
          sent_messages: vec![],
      }
  }
}

impl Messenger for MockMessenger {
  fn send(&self, message: &str) {
      // self.sent_messages.push(String::from(message)); // Borrowing checks failed.
      println!("send message is {}", message);
  }
}

pub fn run() {
  let m = MockMessenger::new();
  let mut tracker = LimitTracker::new(&m, 100);
  tracker.set_value(90);
}