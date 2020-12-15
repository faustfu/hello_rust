#![allow(dead_code,unused_imports)]

// use std::mem;

/// 1. "fn main()" in src/main.rs(library crate) or src/lib.rs(binary crate) is a default entry point/crate root of the project.
/// 2. "fn" is a key word to declare a function.
/// 3. "println!" is a macro.
/// 4. macro is a kind of pseudo code for compiler to generate some code at compile time.
/// 5. ";" indicate the end of a statement. Every statements need a semicolon if it is not a final return statement in a block;

mod apps;
mod cores;
mod functions;
mod types;
mod concurrencies;
mod variables;
mod errors;
mod flows;
mod mods;
mod lif1;
mod lif2;
mod lif3;
// mod mo1;
// mod io1;
// mod io2;
// mod doc1;
mod oo1;
mod oo2;
mod oo3;
mod oo4;
mod own1;
mod own2;
mod own3;
mod own4;
mod patterns;
mod print;
// mod auto1;
// mod tt1;
// mod tt2;

//entry point, no parameters nor return values.
#[tokio::main]
async fn main() {
    // autopilot::key::type_string("Hello, world!", &[], 200., 0.);
    // let _ = autopilot::alert::alert("Hello, world!", None, None, None);
    // auto1::run();
    
    // let a = 1;
    // let b = Box::new(2);

    // println!("Hello, world!");
    // println!("a = {}, size of a = {}", a, mem::size_of_val(&a));
    // println!("b = {}, size of b = {}", b, mem::size_of_val(&b));

    // apps::adt1::run();

    // apps::arg1::run();

    // apps::cr1::run();

    // apps::fail1::run()

    // apps::pow1::run();

    // apps::pow2::run();

    // cores::file1::run();

    // cores::file2::run();

    // errors::err1::run();

    // errors::err2::run();

    // errors::err3::run();

    // flows::cmp1::run();

    // flows::cmp2::run();

    // flows::loop1::while1();

    // flows::loop1::for1();

    // flows::macro1::run();

    // flows::macro2::run();

    // flows::macro3::run();

    // flows::match1::run();

    // flows::pat1::run();

    // flows::pat2::run();

    // patterns::builder1::run();

    // patterns::builder2::run();

    // types::ar1::run();

    // types::enum1::run();

    // types::int1::run();

    // types::it1::run();

    // types::it2::run();

    // types::it3::run();

    // types::it4::run();

    // types::gen1::run();

    // types::gen2::run();

    // types::map1::run();

    // types::option1::run();

    // types::pointer1::run();

    // types::pointer2::run();

    // types::pointer3::run();

    // types::rc1::run();

    // types::rc2::run();

    // types::rc3::run();

    // types::rc4::run();

    // types::rc5::run();

    // types::rc6::run();

    // types::re1::run();

    // types::req1::run().await;

    // types::result1::run();

    // types::sh::run();

    // types::sl1::run();

    // types::str1::run();

    // types::str2::run();

    // types::str3::run();

    // types::trait1::run();

    // types::trait2::run();

    // types::trait3::run();

    // types::trait4::run();

    // types::trait5::run();

    // types::trait6::run();

    // types::tu1::run();

    // types::sets::vec1::run();

    // lif1::lif1();

    // lif1::lif2();

    // lif1::lif3();

    // lif1::lif4();

    // lif1::lif5();

    // lif2::run();

    // lif3::run();

    // mods::mo1::run();

    // functions::clo1::run();

    // functions::clo2::run();

    // functions::clo3::run();

    // functions::clo4::run();

    // functions::clo5::run();

    // functions::clo6::run();

    // functions::clo7::run();

    // functions::clo8::run();

    // functions::fn1::run();

    // functions::fn2::run();

    // io1::run();

    // io2::run();

    // doc1::run();

    // oo1::run();

    // oo2::run();

    // oo3::run();

    // oo4::run();

    // own1::run();

    // own2::run();

    // own3::run();

    // own4::run();

    // print::run();

    // variables::const1::run();

    // variables::static1::run();

    // variables::var1::run();

    // concurrencies::arc1::run();

    // concurrencies::arc2::run();

    concurrencies::async1::run();

    // concurrencies::atom1::run();

    // concurrencies::ba1::run();

    // concurrencies::ch1::run();

    // concurrencies::ch2::run();

    // concurrencies::ch3::run();

    // concurrencies::lock1::run();

    // concurrencies::mu1::run();

    // concurrencies::th1::run();

    // concurrencies::th2::run();

    // tt1::run();

    // tt2::run();
}
