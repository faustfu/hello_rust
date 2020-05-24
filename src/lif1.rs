// 1. Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// 2. The main aim of lifetimes is to prevent dangling references.(lifetime overlaps or is out of scope as accessing)
// 3. A lifetime could be anonymous or named by starting with an apostrophe "'" combining a lexical label after the "&" of a reference.
// 4. Function parameter lifetimes could be generic likes parameter types.
// 5. The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.
//   a. Each parameter that is a reference gets its own lifetime parameter.
//   b. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//   c. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.
//   d. If there are any lifetime unknown parameters after applying previous rules, the compiler will stop with an error.

use std::fmt::Display;

// pub fn lif1() {
//   let r;                // ---------+-- 'a
//                         //          |
//   {                     //          |
//     let x = 5;          // -+-- 'b  |
//     r = &x;             //  |       | //(X)The borrow checker runs here and is failed.
//   }                     // -+       | //   Because r is binding with 'b and 'b is invalid outside of the block.
//                         //          |
//   println!("r: {}", r); //          |
// }                       // ---------+

// pub fn lif2() {
//   let x = 5;            // ----------+-- 'b
//                         //           |
//   let r = &x;           // --+-- 'a  | //(O)The borrow checker runs here and is passed.
//                         //   |       |
//   println!("r: {}", r); //   |       |
//                         // --+       |
// }                       // ----------+

pub fn lif3() {
  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}

// // case 1
// pub fn lif4() {
//   let string1 = String::from("long string is long");
//   let result;
//   {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str()); // new lifetime is based on string2 and is shorter than result.
//   }
//   println!("The longest string is {}", result);
// }

// The lifetime of the reference returned by the longest function is the same as
//  the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// case 2
pub fn lif5() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Hey"); // new lifetime is based on string2 and is shorter than result.
    println!("The longest string is {}", result);
  }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}