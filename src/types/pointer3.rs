// 1. Any values implemented trait:Drop will be called drop() when they went out of the scope.
// 2. Dropping is done recursively.
// 3. Variables are dropped in reverse order of declaration.

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

pub fn pointer3() {
  // case 1
  let _c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let _d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");

  // case 2(drop by force)
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}
