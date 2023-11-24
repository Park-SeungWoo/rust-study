pub fn results_fn() {
    let divide = divide(4, 2);
    // let res = divide.expect("we crushed!");  // Will throw an exception if result is an Error type

    /*
    3 ways to extract value from Result object.
    1. match statment
    2. .is_ok() method with if statement
    3. .unwrap(), .unwrap_or(default), etc..
    Be sure we cannot use these three ways in a same scope.
    Because if we use divide variable in each statement, the ownership will move into that scope.
     */
    // match divide {
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v),
    // }

    // if divide.is_ok() {
    //     println!("{}", divide.unwrap());
    // }

    println!("{}", divide.unwrap());
    // println!("{}", divide.unwrap_or(100));

    // println!("{}", res);
}

/*
Err, and enum that contains an error code
Ok(value), A wrapper that contains a value
 */
fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

#[derive(Debug)]  // to print enum or struct in terminal, we have to add this line to let the comiler know that this object is printable.
enum MyError {
    Error1
}