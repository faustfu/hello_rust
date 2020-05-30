// 1. Traits is like interfaces in other languages.
// 2. We can use trait bounds to specify that a generic can be any type that has certain behavior.
// 3. Traits may be public or private, but methods in traits is public by default.
// 4. The orphan rule : we can implement a trait on a type only if either the trait or the type is local to our crate.
// 5. A generic type parameter can only be substituted with one concrete type at a time.(zero-cost abstraction)

trait Summary {
  fn summarize(&self) -> String; // declarations
  fn summarize_author(&self) -> String;
  fn sum(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  } // default implementation
}

struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

#[derive(Debug)]
struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// declare a generic function bounded with a trait.
fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize()); // use trait methods
}

use std::fmt::Debug;

// declare a generic function bounded with two traits.
fn print<T: Summary + Debug>(item: &T) {
  println!("Breaking news! {:?}", item);
}

fn dump<T>(item: &T) where T: Summary + Debug { // reduce duplicated declarations of traits by "where"
  println!("Breaking news! {:?}", item);
}

fn returns_summarizable() -> impl Summary + Debug {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
  }
}

pub fn run() {
  // case 1(define method interfaces in "trait" block and implement method body in types)
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());

  // case 2(define methods with default implementations in "trait" block)
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
    ),
  };
  println!("2 new article available! {}", article.sum());

  // case 3(use a generic type as function parameters)
  notify(&article);

  // case 4(use multiple generic types as function parameters)
  print(&tweet);

  // case 5(use "where" to reduce function signature)
  dump(&tweet);

  // case 6(use traits for returning generic types)
  println!("6 new tweet: {:?}", returns_summarizable());
}
