// 1. Generic types <-> Concrete types.

pub fn gen1() {
  // case 1(raw)
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = number_list[0];

  for number in number_list {
      if number > largest {
          largest = number;
      }
  }

  println!("The largest number is {}", largest);

  // case 2(use functions to reduce duplicated codes)
  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  // pass a vector as an array.
  let result = biggest(&number_list);
  println!("The largest number is {}", result);

  // case 3(two raw functions)
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest_i32(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest_char(&char_list);
  println!("The largest char is {}", result);

  // case 4(use a generic function to reduce duplicated codes)
  let number_list = vec![34, 50, 25, 100, 65];

  let result = bigg(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = bigg(&char_list);
  println!("The largest char is {}", result);
}

fn biggest(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn bigg<T:Copy + PartialOrd>(list: &[T]) -> T {
  let mut largest = list[0]; // Copy

  for &item in list.iter() {
      if item > largest { // PartialOrd
          largest = item;
      }
  }

  largest
}
