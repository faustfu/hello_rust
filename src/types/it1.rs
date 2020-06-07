// ref: https://doc.rust-lang.org/std/iter/index.html
/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
*/

use std::collections::HashMap;

pub fn run() {
  // case 1
  let a = [1, 2, 3].iter();
  let b = [1, 2, 3].iter();

  for (i, v) in a.enumerate() {
    println!("index = [{}], value = [{}]", i, v);
  }

  for &v in b {
    println!("item = [{}]", v);
  }

  // case 2(iter)
  let c = vec![1, 2, 3];

  let mut v2_iter = c.iter(); // The iterator is mutable, elements are immutable.

  assert_eq!(v2_iter.next(), Some(&1));

  // case 3(into_iter)
  let d = vec![1, 2, 3];

  let mut v3_iter = d.into_iter(); // The iterator is mutable, and it has ownership of elements.

  assert_eq!(v3_iter.next(), Some(1));

  // case 4(iter_mut)
  let mut e = vec![1, 2, 3];

  let mut v4_iter = e.iter_mut(); // The iterator is mutable, elements are mutable.

  assert_eq!(v4_iter.next(), Some(&mut 1));

  // case 5(map)
  let f = vec![1, 2, 3];
  for v in f.iter().map(|x| x + 1) {
    println!("item = [{}]", v);
  }

  // case 6(collect)
  let g = vec![1, 2, 3];

  let v6_vec: Vec<_> = g.iter().map(|x| x + 1).collect();

  assert_eq!(v6_vec, vec![2, 3, 4]);

  // case 7(map+filter+for_each)
  let v7_vec = vec![1, 2, 3];
  v7_vec
    .iter()
    .map(|x| x + 1)
    .filter(|x| x > &2)
    .for_each(|x| println!("gogo {}", x));

  // case 8(chain+enumerate)
  // chain two iterators and show final pairs
  let v8_vec = vec![1, 2, 3];
  for (i, v) in v8_vec.iter().chain(Some(42).iter()).enumerate() {
    println!("{}: {}", i, v);
  }

  // case 9(collect)
  let v9_vec = vec![1, 2, 3];
  let v9_vec_2: Vec<_> = v9_vec.iter().map(|x| x * 2).collect();
  println!("collected vec is {:?}", v9_vec_2);
  let v9_map: HashMap<_, _> = v9_vec.iter().map(|x| x * 3).enumerate().collect();
  println!("mapped vec is {:?}", v9_map);
}

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];

  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum(); // like for loop, sum() will take ownership of the iterator.

  assert_eq!(total, 6);
}
