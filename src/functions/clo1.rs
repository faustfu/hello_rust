
// How a value is captured by a closure will imply the same set of borrow rules we’re used to in Rust.
// 1. If by reference, then other references can live concurrently with the closure.
// 2. If by mutable reference, then as long as the closure is alive, no other references to the values can exist.
//    However, once the closure is dropped, other references can exist again.
// 3. If by value, then the value cannot be used by anything ever again.
//    (This automatically implies that the closure owns the value.)
// 4. If by reference(mutable or not), lifetime of the reference should be longer than the closure's.
// 5. Using the move keyword like this moves all captured variables into the closure,
//     and therefore they cannot be used after the closure.
// 6. Closures will preferentially capture by immutable reference, then by mutable reference, and only then by value.
// 7. To solve lifetime issues, we can force a closure to capture by value with the move keyword.
// 8. Regarding the traits of closures:
//    a. If a closure uses anything by value, then the closure is a FnOnce.
//    b. Otherwise, if a closure uses anything by mutable reference, then the closure is a FnMut,
//        which automatically implies FnOnce as well.
//    c. Otherwise, a closure is a Fn, which automatically implies both FnMut and FnOnce.

// normal functions, returning values depend on parameters.
fn inc(i:i32) -> i32 {
  i + 1
}

pub fn clo1() {
  // case 1(copy)
  let i:i32 = inc(1);

  // closures, returning values depend on parameters and context.
  let clo1 = |j:i32| j + i; // i is fixed to 2 in the closure.

  println!("inc(1) is {}", i);
  println!("clo(2) is {}", clo1(2));
  println!("clo(5) is {}", clo1(5));

  // case 2(move ownership by value)
  let x = String::from("Alice 1");
  let clo2 = || {
    let y = x; // y is binding the value.
    println!("Hello, {}", y);
    // the lifetime of ownership is ended here.
  };
  clo2();
  // clo2();  // Cannot use x again.

  // case 3(by reference)
  let x = String::from("Alice 2");
  let clo3 = || {
    let y = &x; // immutable borrow
    println!("Hello, {}", y);
    // the lifetime of borrowed reference is ended here.
  };
  clo3();
  clo3(); // it is valid to use x again.

  // case 4(move ownership by using "move" keyword)
  let name = String::from("Alice 3");
  let _ = move || println!("Hello, {}", name);
  // println!("Using name from main: {}", name); // invalid access here!
}