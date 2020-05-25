#![allow(dead_code)]

extern crate autopilot;
// use std::mem;

/// 1. "fn main()" in src/main.rs(library crate) or src/lib.rs(binary crate) is a default entry point/crate root of the project.
/// 2. "fn" is a key word to declare a function.
/// 3. "println!" is a macro.
/// 4. macro is a kind of pseudo code for compiler to generate some code at compile time.
/// 5. ";" indicate the end of a statement. Every statements need a semicolon if it is not a final return statement in a block;

mod functions;
mod types;
mod concurrencies;
mod variables;
mod errors;
mod flows;
mod mods;
mod lif1;
mod lif2;
// mod mo1;
// mod io1;
// mod io2;
// mod cr1;
// mod doc1;
mod own1;
mod own2;
mod own3;
mod own4;
// mod pointer1;
// mod auto1;
// mod trait1;
// mod tt1;
// mod tt2;

//entry point, no parameters nor return values.
fn main() {
    // autopilot::key::type_string("Hello, world!", &[], 200., 0.);
    // let _ = autopilot::alert::alert("Hello, world!", None, None, None);
    // auto1::sine_mouse_wave();
    
    // let a = 1;
    // let b = Box::new(2);

    // println!("Hello, world!");
    // println!("a = {}, size of a = {}", a, mem::size_of_val(&a));
    // println!("b = {}, size of b = {}", b, mem::size_of_val(&b));

    // conditions::con1();

    // errors::err1::err1();

    // errors::err2::err2();

    // errors::err3::err3();

    // flows::cmp1::cmp1();

    // flows::cmp2::cmp2();

    // flows::loop1::while1();

    // flows::loop1::for1();

    // flows::match1::match1();

    // flows::pat1::pat1();

    // types::ar1::ar1();

    // types::enum1::enums();

    // types::it1::it1();

    // types::gen1::gen1();

    // types::gen2::gen2();

    // types::map1::map1();

    // types::option1::op1();

    // types::sh::heap();

    // types::sl1::slice1();

    // types::str1::str1();

    // types::str2::str2();

    // types::tu1::tu1();

    // types::vec1::vec1();

    // lif1::lif1();

    // lif1::lif2();

    // lif1::lif3();

    // lif1::lif4();

    // lif1::lif5();

    // lif2::case1();

    // mods::mo1::mo1();

    // functions::clo1::clo1();

    // functions::clo2::clo2();

    // functions::clo3::clo3();

    // functions::clo4::clo4();

    // functions::clo5::clo5();

    functions::clo6::clo6();

    // functions::fn1::fn1();

    // io1::io1();

    // io2::io2();

    // cr1::cr1();

    // doc1::doc1();

    // own1::own1();

    // own2::own2();

    // own3::own3();

    // own4::own4();

    // pointer1::pointer1();

    // variables::const1::const1();

    // variables::var1::var1();

    // concurrencies::th1::th1();

    // concurrencies::th2::th2();

    // concurrencies::th3::th3();

    // concurrencies::th4::th4();

    // concurrencies::th5::th5();

    // trait1::trait1();

    // tt1::tt1();

    // tt2::tt2();
}
