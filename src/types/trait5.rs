// 1. It is possible to have same function names in different traits.
//    And it is ok to implement those traits in a struct at the same time.
// 2. Raw method/function signature of traits:
//    <Type as Trait>::function(receiver_if_method, next_arg, ...);

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
      String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
      String::from("puppy")
  }
}

pub fn run() {
  // case 1(call default struct implemented methods)
  let person = Human;
  person.fly();

  // case 2(call trait implemented methods)
  Pilot::fly(&person); // use raw function signature with self instance
  Wizard::fly(&person);

  // case 3(call trait associated functions)
  println!("A baby dog's name is {}", Dog::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
