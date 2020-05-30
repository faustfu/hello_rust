// 1. slice is a reference/view of the collection, therefore it could not get ownership of original collection.
// 2. Internally, the slice data structure stores the starting position and the length of the slice=(end-start) index).
// 3. signatures
//    slice of String => &str
//    slice of array/vector => &[<element type>]
// 4. string literals' type is &str.

fn show_slice(input:&[i32]) {
  println!("slice is {:?}, len of slice is {}", input, input.len());
}

fn change_array_by_slice(input:&mut[i32]) {
  input[1] = 6;
}

pub fn run() {
  let mut a = [1,2,3,4,5];
  
  show_slice(&a[1..3]);
  change_array_by_slice(&mut a[1..3]);
  show_slice(&a);
}