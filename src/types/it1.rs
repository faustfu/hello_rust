// ref: https://doc.rust-lang.org/std/iter/index.html
pub fn it1() {
  let a = [1, 2, 3].iter();
  let b = [1, 2, 3].iter();

  for (i, v) in a.enumerate() {
    println!("index = [{}], value = [{}]", i, v);
  }

  for &v in b {
    println!("item = [{}]", v);
  }
}
