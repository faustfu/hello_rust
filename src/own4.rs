// 1. Named lifetimes are a way of giving these scopes a name. 
// 2. 'a is just a meaningless lifetime declaration.
// 3. 'static means that something has the lifetime of the entire program.
// 4. elided lifetime for function parameters(input) and return values(output)
//    a. every reference parameters has lifetime, elided or not.
//    b. elided output lifetime is decided by input lifetime.
//       If there is no any input lifetime or multiple input lifetime, elided output lifetime is illegal.
//    c. if the function is a object's method, elided output lifetime is same with the object.
struct Foo<'a> {
    x: &'a i32,
}

pub fn own4() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here:borrowed value `f.x` does not live long enough
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
} 