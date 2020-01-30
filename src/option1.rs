// 1. option is generic enum of Some and None. Therefore it could be some value or none.
// enum Option<T> {
//     Some(T),
//     None,
// }
// 2. Choosing between match and if let depends on what you’re doing in your particular situation and
//     whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

pub fn op1() {
  let x = 3.0;
  let y = 2.0;
  let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };

  println!("result = {:?}", result);

  match result {
    Some(z) => println!("{} / {} = {}", x, y, z),
    None => println!("{} / {} has errors", x, y)
  }

  if let Some(z) = result { println!("z = {}", z) };
}
