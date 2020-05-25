// 1. Use trait objetcs to share common codes in multiple concrete types at runtime.
// 2. The trait has to be object-safe to implement trait objects.
// 3. A trait is object safe if all the methods defined in the trait have the following properties:
//      a. The return type isn’t Self.
//      b. There are no generic type parameters.

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // use "Box" and "dyn" to deal with uncertain types.
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Button {} is drawing...", self.label);
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("SelectBox is drawing...");
    for v in self.options.iter() {
      println!("\tOption {} is drawing...", v);
    }
  }
}

pub fn oo2() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}
