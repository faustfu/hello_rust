// 1. A reference variable could be accessed to the value by using dereference operator "*".
// 2. In some functions, Rust could recognize a variable is a reference and dereferer it automatically.
// 3. Deref coercion converts such a type into a reference to another type.
// 4. Rust does deref coercion when it finds types and trait implementations in three cases:
//      a. From &T to &U when T: Deref<Target=U>
//      b. From &mut T to &mut U when T: DerefMut<Target=U>
//      c. From &mut T to &U when T: Deref<Target=U>

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T; // Register the associated type.

  fn deref(&self) -> &T {
    &self.0 // Return real reference, then it could be accessed by dereference operator.
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

pub fn run() {
  // case 1(normal reference)
  let a = &5;
  let b = &5;
  println!("a is {}, b is {}", *a, b);
  assert_eq!(5, *b); // Assert statements would not dereferer a reference automatically

  // case 2(Box)
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y); // *(y.deref())

  // case 3(MyBox)
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);

  // case 4(Deref coercion chain)
  let m = MyBox::new(String::from("Rust"));
  hello(&m); // *(m.deref()) // Deref coercion happens here. 1)&MyBox<String> => &String, 2)&String => &str
}
