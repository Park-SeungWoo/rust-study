pub fn mutability_fn() {
    // every type is immutable (cannot change)
    let num = 5;
    // num = 3;  // err
    println!("{}", num);

    let mut num2 = 5;  // mutable
    println!("initial: {}", num2);
    num2 = 3;
    println!("changed: {}", num2);
}