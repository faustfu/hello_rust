// 1. structs could have reference types. Their lifetimes may be different, so lifetime annotations is needed, too.

#[derive(Debug)]
struct ImportantExcerpt<'a> { //An instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {}", announcement);
      self.part
  }
}

pub fn run() {
  // case 1
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("case 2-1)i is {:?}", i);

  // case 2: The compiler will decide lifetime of the output parameter.
  println!("case 2-2)announce is {:?}", i.announce_and_return_part("gogogo"));
}
