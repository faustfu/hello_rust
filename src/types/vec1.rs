// 1. Vectors use "usize" as index type and the index starts from 0.
// 2. Vectors keep same type values into heap and could use push or pop methods as a stack to change size.
// 3. A vector and its values will be dropped as leaving the scope.
// 4. Length of vector is the number of current elements in the vector.
//    access method: <vector>.len()
// 5. Capacity of vector is the number of current space in the vector.
//    access method: <vector>.capacity()
//    initial method: Vec::with_capacity(<size>)

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn run() {
  let mut a = Vec::new(); // Vectors are implemented using generics. It may be assigned a type as initialization.
  let b = vec![1, 2, 3]; // vector macro for creating a vector with initial values.

  for x in 1..5 {
    //1 <= x < 5
    a.push(x);
  }

  println!("a is {:?}", a);
  println!("b is {:?}, b[1] is {}", b, &b[1]); // return a reference of the value or panic if the value is not exists.

  // get() returns Option to prevent out of index boundry error
  match a.get(4) {
    // return a option of the value
    Some(x) => println!("x = {}", x),
    None => println!("none"),
  }

  // loop by element readonly references
  for x in &a {
    println!("{}", x);
  }

  // change by element mutable references
  for x in &mut a {
    *x += 2
  }
  println!("a is {:?}", a);

  // push a element into the vector
  a.push(8);
  println!("a is {:?}", a);

  // take a iterator with its ownership to extend the vector
  a.extend([1, 2, 3].iter().cloned());
  println!("a is {:?}", a);

  // read by loop
  for i in a.iter() {
    println!("{}", i);
  }

  // write by loop
  for i in a.iter_mut() {
    *i += 1;
  }

  a.pop(); //pop() will return Option
  println!("a is {:?}", a);

  // move all values out of the vector
  while let Some(x) = a.pop() {
    println!("final value = {}", x);
  }

  // use enum to store values of different types in a vector.
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];

  for x in &row {
    println!("{:?}", x);
  }
}
