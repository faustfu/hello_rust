// 1. Every states have some common methods to change to other states.

// inner objects
trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {}) // change to new state:PendingReview
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self // state unchanged
  }
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self // state unchanged
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content // It is valid to show post content now.
  }
}

// public interfce
pub struct Post {
  state: Option<Box<dyn State>>, // Keep current state
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})), // start from state:Draft
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    // &self.content
    self.state.as_ref().unwrap().content(self) // Use as_ref() to get current state.
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

pub fn oo3() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}
