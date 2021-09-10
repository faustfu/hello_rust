// 1. Lifetime annotations of function parameters are not required in some conditions.
//   a) Only one parameter => Output lifetime is input lifetime.
//   b) The function is a method => Output lifetime is object lifetime.
//   c) No output values.

fn case1(x: &u64) -> &u64 {
    println!("{}", x);
    x
}

struct Man {
    age: u64
}

impl Man {
    fn case2(&self, x: &u64) -> &u64 {
        println!("{} {}", self.age, x);
        &self.age
    }
}

fn case3(x: &u64, y:&u64) {
    println!("{} {}", x, y);
}

pub fn run() {
    let a = 1;
    case1(&a);

    let b = 2;
    let c = Man {
        age: 3,
    };

    c.case2(&b);

    case3(&a, &b);
}
