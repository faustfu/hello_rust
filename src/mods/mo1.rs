// Packages: A Cargo feature that lets you build, test, and share crates.
//           A package is one or more crates that provide a set of functionality.
//           A package contains a Cargo.toml file that describes how to build those crates.
//           A package must contain zero or one library crates, and many binary crates as you’d like.
// Crates: A tree of modules that produces a library or executable(binary).
//         A crate is a binary(src/main.rs) or library(src/lib.rs). The crate root is a source file that
//          the Rust compiler starts from and makes up the root module of your crate section.
//         Only library crates expose functions that other crates can use.
// Modules: The structure of a crate. Let you control the organization, scope, and privacy of paths.
//          Files or folders in src folder are private modules.
//          Two kinds of modules:
//            1. Inline modules: Declare header with implementation in a file or module.
//            2. File modules: Split implementation from the header.
//               Declare implementation in another file(<module name>.rs) or a folder(<module name>/mod.rs).
//          The "use" keyword that shorten a path in this scope.
//          The "pub" keyword to make items public.
//          The "extern crate" keyword to import an external crate in this scope.
//          The "pub use" keyword to re-export a path for external usage.
// Paths: A way of naming an item, such as a struct, function, or module.
//        Identifiers in a path are separated by double colons "::".
//        a. absolute path: start from the root of a crate by using a crate name or a literal "crate" in the same crate.
//        b. relative path: start from the current module and uses "self", "super", or an identifier in the current module.


// namespace
// 1. hierarchy: crate::module::item, ex: std::mem::size_of_val


use std::collections::LinkedList; // create an shortcut in this scope.

// declare inline modules
mod hello {
  pub fn say_hello() {
    println!("hello");
  }
}

pub fn run() {
  let mut a = LinkedList::new(); // use the shortcut
  a.push_back(1);
  a.push_back(2);

  println!("a is {:?}", a);

  for a1 in a {
    println!("a1 is {}", a1);
  }

  let mut v = Vec::new(); // full path => std::collections::Vec;
  v.push(3);
  v.push(4);

  println!("v is {:?}", v);

  for v1 in v {
    println!("v1 is {}", v1);
  }

  hello::say_hello(); // use internal crates
}
