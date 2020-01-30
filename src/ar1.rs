// array len() is fixed.
// array use "usize" as index type
// the length of array is fixed.
pub fn ar1() {
  let mut x:[i32;5] = [1,2,3,4,2]; 

  x[1] = 3;

  println!("x = {:?}, len = {}", x, x.len());

  let b = [1u32; 10];

  for i in b.iter() {
    println!("i = {}", i);
  }

  for i in &b {
    println!("i = {}", i);
  }

  //two dimension array
  let c:[[i32;3];2] = [
    [1,2,3],
    [4,5,6]
  ];

  println!("c = {:?}", c);
}