// 1. Iterator has many adaptors to convert its behaviors.
// 2. Adaptors are lazy.
use std::collections::HashMap;

pub fn run() {
  // case 1(map)
  let f = vec![1, 2, 3];
  for v in f.iter().map(|x| x + 1) {
    // map() return a new iterator with new sets.
    println!("item = [{}]", v);
  }

  // case 2(collect)
  let g = vec![1, 2, 3];

  let v2_vec: Vec<_> = g.iter().map(|x| x + 1).collect();

  assert_eq!(v2_vec, vec![2, 3, 4]);

  // case 3(map+filter+for_each)
  let v3_vec = vec![1, 2, 3];
  v3_vec
    .iter()
    .map(|x| x + 1)
    .filter(|x| x > &2)
    .for_each(|x| println!("gogo {}", x));

  // case 4(chain+enumerate)
  // chain two iterators and show final pairs
  let v4_vec = vec![1, 2, 3];
  for (i, v) in v4_vec.iter().chain(Some(42).iter()).enumerate() {
    println!("{}: {}", i, v);
  }

  // case 5(collect)
  let v5_vec = vec![1, 2, 3];
  let v5_vec_2: Vec<_> = v5_vec.iter().map(|x| x * 2).collect();
  println!("collected vec is {:?}", v5_vec_2);
  let v9_map: HashMap<_, _> = v5_vec.iter().map(|x| x * 3).enumerate().collect();
  println!("mapped vec is {:?}", v9_map);
}
