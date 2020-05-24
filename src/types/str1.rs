// 1. two types of string
//    a. "String" is a mutable char/byte/u8 vector(changable) in heap.
//    b. "str" is a literal(fixed) string in core language.
//       normal borrowed form: "&str". a literal string reference for static lifetime: "&'static str".
// 2. Both String and string slices(&str) are UTF-8 encoded.
// 3. There are many ways to access a string inside.
//    a. to get a string slice(&str), and it will panic if the range index is not valid nor a char boundary: &<name>[<range>]
//    b. to get elements in a string by bytes: <name>.bytes()
//    c. to get elements in a string by chars: <name>.chars()

pub fn str1() {
  let s1: &'static str = "123"; //static str is inline readonly text
  let s2: &str = "456"; //&str = string slice
  let s3 = "789"; // same as s2. Default type is &str

  println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

  for c in s1.chars() {
    println!("c is {}", c);
  }

  if let Some(c1) = s1.chars().nth(0) {
    println!("first letter is {}", c1);
  }

  //setup a string by String object
  let mut letters = String::new(); //declare an empty string object for later usage.
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
    letters.push(a as char); //append a char
    letters.push_str(","); //append a str
    letters += " "; //same as push_str() to concatenate text. But it will consume self ownership!
    a += 1;
  }
  println!("letters is {}", letters);

  let t1 = String::from("tic");
  let t2 = String::from("tac");
  let t3 = String::from("toe");
  let t = format!("{}-{}-{}", t1, t2, t3); // concatenate strings with complicated formation by format macro.
  println!("game is {}", t);

  //convert Strong object to &str
  let u: &str = &letters;
  println!("u is {}", u);

  //convert str to String object
  let v = "v is {}";
  println!("v is {}", v.to_string());

  //inital String object from &str
  let w = String::from("w is wu");
  println!("w is {}", w);

  let mut a1 = String::from("foo");
  let mut a2 = "bar";
  a1.push_str(a2);
  a2 = "wo"; // assign a new str reference
  println!("a1 is {}", a1);
  println!("a2 is {}", a2);
}
