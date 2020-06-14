// 1. Largest built-in integer type Rust is u128. 
pub fn run() {
  // case 1
  println!("hex of 0x4e00 is {}", 0x4e00);
  println!("hex of 0x4e004e00 is {}", 0x4e004e00);
  println!("hex of 0x4e004e004e004e00 is {}", 0x4e004e004e004e00 as u64);
  println!("hex of 0x4e004e004e004e004e004e004e004e00 is {}", 0x4e004e004e004e004e004e004e004e00 as u128);
}