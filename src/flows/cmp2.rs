// IF
// 1. the type of if evaluation result(condition) has to be boolean.
// 2. if there are more than one conditions is matched, Rust will just run the first matched block.
// 3. "if" is a expression. So every blocks should return same type values.
pub fn cmp2() {
  let a = 2;
  let b = if a == 1 {"a == 1"} else {"a != 1"}; // 

  println!("a = {}, b = {}", a, b);
}
