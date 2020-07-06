// 1. Iterator has many comsumers to do sth by its data.
//
pub fn run() {
  // case 1(any)
  let a = [1, 2, 3];
  assert_eq!(a.iter().any(|&x| x != 2), true);

  // case 2(fold)
  let b = [4,5,6];
  let sum = b.iter().fold(0, |acc, &x| acc + x);
  println!("sum is {}", sum);

  // case 3(collect)
  let c = [7,8,9];
  let coll = c.iter().collect::<Vec<&i32>>();
  println!("coll is {:?}", coll);
}
