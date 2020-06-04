use regex::Regex;

pub fn run() {
  let text = "faust";

  // case 1(match)
  let re = Regex::new(r"\w{5}").unwrap();
  println!("Am I matched? {}", re.is_match(text));

  // case 2(match group)
  let re = Regex::new(r"(\w{5})").unwrap();
  match re.captures(text) {
    Some(caps) => println!("First matched content is {}", &caps[0]), // caps.get(0).unwrap().as_str()
    None => println!("Not found")
  }
}