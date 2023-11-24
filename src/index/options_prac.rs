use std::collections::HashMap;

pub fn options_fn() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());  // unwrap() => Will extract the actual value
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());  // If we unwrap a None, It'll panic! i.e It'll throw an exception
}

/*
None, to indicate failure or lack of value, and
Some(value), a tuple struct that wraps a value with type T. -> value will be the actual value item in key-value pair.
*/
fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}