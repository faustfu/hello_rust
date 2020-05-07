// home page
https://www.rust-lang.org/zh-TW/
//doc
https://doc.rust-lang.org
//online editor
https://play.rust-lang.org
//crates/modules
https://crates.io/
//doc of crates
https://docs.rs/<crate>

// install compiler and tools
curl https://sh.rustup.rs -sSf | sh

// install rustfmt
rustup component add rustfmt --toolchain stable-x86_64-apple-darwin
// install rls for editor
rustup component add rls
// upgrade rust version
rustup update stable

// format specified file
rustfmt <file path>
// or format package files(no dead codes)
cargo fmt

//generate new package
cargo new hello_world --bin

//build package to target/debug
cargo build

//build package to target/release with optimizations
cargo build --release

//build and run package
cargo run

//run test cases
cargo test

//use compiler to check codes
cargo check

// package structure
Cargo.toml
/src/main.rs // entry point

//// variables
//declaration
1. with specified type
  let [mut] <name>:type [= <value>];
2. assign type by initial value type
  let [mut] <name> = value;

//// trait => interface
//// crate => module/namespace

//// macors
// println
println!()

// panic
panic!()

// Vec::new()
vec![]

//// two error handling types
// 1. return Option[Some(), None] //success/fail
// 2. return Result[Ok(), Err()] //success/fail1/fail2/...
// 3. return doSth()? ~= match doSth() { Some(x) => x, Err(e) => e }

// install rust in juypter
1. install cmake and setup the path
2. cmd>cargo install evcxr_jupyter
3. cmd>evcxr_jupyter --install
