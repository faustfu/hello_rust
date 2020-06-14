// 1. String has many methods working with byte index.
//    Those methods would panic or work improperly if the byte index(usize) is not in legal character boundary.
//    ex: split_at, insert_str, remove, truncate, drain, ...
pub fn run() {
  // case 1(use len() to get byte length of the string)
  let a1 = String::from("一二三");
  println!("length of [一二三] is {}", a1.len());

  // case 2(use find() to get index of the char)
  let a2 = String::from("一二三五");
  println!("index of 三 in [一二三五] is {}", a2.find('三').unwrap_or(a2.len()));

  // case 3(format unicode)
  println!("[{:5}]", "th\u{e9}");
}