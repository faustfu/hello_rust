// 1. A simple way to avoid wrong workflows.
// 2. Use ownership to keep only one instance at a time during the workflow.

// public interfce
pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String,
}

pub struct PendingReviewPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    // return a draft post to prevent accessing to content.
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    // There is only one way to access content outside.
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    // return a pending post with draft content to be approved.
    PendingReviewPost {
      content: self.content,
    }
  }
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    // return a normal post.
    Post {
      content: self.content,
    }
  }
}

pub fn oo4() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review();

  let post = post.approve();

  assert_eq!("I ate a salad for lunch today", post.content());
}
