// 1. Use cfg:test to bypass compile the module as building.
#![allow(dead_code)]

pub fn run() {
    println!("tt2");
}

pub fn add_two(a: i8) -> i8 {
    a + 2
}

// utilities
fn add(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // use everything from upper modules, includes private functions;

    #[test]
    fn it_works() {
        assert_eq!(4, add(2, 2));
    }
}
