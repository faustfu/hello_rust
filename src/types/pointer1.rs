// 1. References are a kind of pointers to borrow the value they point to.
// 2. Smart pointers are data structures that have additional metadata and capabilities.
// 3. Smart pointers have a counter to count references and ownership of the value.
// 4. String and Vec<T> are smart pointers.
// 5. Smart pointers implement the Deref and Drop traits.
// 6. The Deref trait allows an instance to behave like a reference.
// 7. The Drop trait indicates how to destroy an instance goes out of scope.
// 8. Use Box<T> to store data on the heap.

use std::mem;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum List {
    Cons(i32, Box<List>), // normal element type
    Nil, // end element type
}

use List::{Cons, Nil};

fn get_stack_size<T>(value: &T) -> usize {
    mem::size_of_val(value)
}

pub fn run() {
    // case 1(normal Box)
    let a = Box::new(5);
    println!("a = {}", a); // Box implements Deref trait to be able to be unboxed automically

    // case 2(wrap an recursive type value)
    // Rust determine the maxinum size from every fields and use it as the default size.
    let m1 = Message::Quit;
    println!("size of m1 = {}", get_stack_size(&m1));
    let m2 = Message::Move { x: 5, y: 10 };
    println!("size of m2 = {}", get_stack_size(&m2));
    let m3 = Message::Write(String::from("hello"));
    println!("size of m3 = {}", get_stack_size(&m3));
    let m4 = Message::ChangeColor(22, 44, 66);
    println!("size of m4 = {}", get_stack_size(&m4));

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("size of list = {}", get_stack_size(&list));
}
