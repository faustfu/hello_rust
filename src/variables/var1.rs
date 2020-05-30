// 1. use "let" to declare a immutable variable(binding) inside of functions.
// 2. use "let mut" to declare a variable.
// 3. variable has to be initialized before reading.
// 4. built-in(primitive) types:
//   a. scalar types:
//     booleans: bool, key words:true, false.
//     characters: char, contains unicode code(4 bytes).
//     integers: i8/u8, i16/u16, i32/u32, i64/u64, isize/usize(size determined by the machine),
//     floating-point numbers: f32, f64(default floating type)
//   b. compound types:
//     arrays: [<type of elements>;<length of elements>], access syntax: <array name>[<index of the element, ex: 0,1,2...>]
//     tuples: (<type 1>[,<type 2>]...), access syntax: <tuple name>.<index of the element, ex: 0,1,2...>, the elements of a tuple could be destructed to be seperated variables.
// 5. cast: <var name> as <type name>

fn p_str(a:&str) {
  println!("This is {}", a);
}

fn p_bool(a:bool) {
  println!("This is {}", a);
}

fn p_char(a:char) {
  println!("This is {}", a);
}

fn p_int(a:i32) {
  println!("This is {}", a);
}

pub fn add(a:i32,b:i32) -> i32 {
  return a + b;
}

pub fn run() {
  let a = "asd";
  let b = false;
  let c = '嗨'; // use single quota to declare a character.

  p_str(a);
  p_bool(b);
  p_char(c);
  p_int(add(1, 2));
}