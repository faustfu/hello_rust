pub fn run() {
  // position arguments
  println!("{0} {1} {0} {2}", 1,3,4);

  // amed arguments
  println!("{who}怕{what}?", who = "你", what = "什麼");

  // number base
  println!("10 = {0:b} {0:x} {0:o}", 10);

  // debug trait, print whole object structure includes values.
  println!("value is {:?}", (false, 10, "hi"));
}