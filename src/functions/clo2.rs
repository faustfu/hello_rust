// 1. closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// 2. Closures can capture values from the scope in which they’re defined.
// 3. Closures are used for the cases which want to seperate executions from initial processes.
// 4. Type annotations are optional for closures if compiler could inferer types from their usage.

use std::thread;
use std::time::Duration;

pub fn run() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
  // case 1
  let expensive_closure_1 = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  }; // expensive_closure is a definition of logic, not an execution result.

  // case 2(type annotations)
  let expensive_closure_2 = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  // case 3(shorten block)
  let _add_one_v3 = |x:u32| {x + 1};

  // case 4(shorten block)
  let _add_one_v4 = |x:u32| x + 1;

  if intensity < 25 {
    let expensive_result = expensive_closure_1(intensity); // run the codes as needed.

    println!("Today, do {} pushups!", expensive_result);
    println!("Next, do {} situps!", expensive_result);
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!("Today, run for {} minutes!", expensive_closure_2(intensity));
    }
  }
}
