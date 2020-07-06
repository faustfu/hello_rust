// ref: https://doc.rust-lang.org/std/iter/index.html
/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
*/

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

  // println!("d is {:?}", d); // It is failed because d is moved.

  // case 4(iter_mut)
  let mut e = vec![1, 2, 3];

  let mut v4_iter = e.iter_mut(); // The iterator is mutable, elements are mutable.

  assert_eq!(v4_iter.next(), Some(&mut 1));
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
