pub fn basics_fn() {
    /*
    primitive
     */
    // integers
    //// unsigned
    let unsigned: u8 = 10;

    //// signed
    let signed: i8 = -10;

    // floats
    let float: f32 = 1.2;

    println!("u: {}, s: {}, f: {}", unsigned, signed, float);

    // char
    let letter = "c";  // str
    let c = 'c';  // char
    let emoji = "\u{1F600}";
    println!("letter: {}, char: {}, emoji: {}", letter, c, emoji);

    // bool
    let is_true: bool = true;

    println!("isTrue: {}", is_true);

    /*
    non-primitive
     */
    println!();
    // array
    let arr: [u8; 3] = [1, 2, 3];
    let arr2: [u8; 5] = [100; 5];  // [100 100 100 100 100]

    println!("index: {}, length: {}", arr2[0], arr2.len());

    println!("arr: {:?}, arr2: {:?}", arr, arr2);  // print objects' structures i.e. print entire elements like print(list) in python

    // tuple
    let tuple: (u8, bool, f32) = (5, true, 1.3);  // must match the elements' order with the order of types specified in declaration
    let tuple2 = (5, 3, true, 1.3, false);

    println!("fisrt idx: {}, second: {}, third: {}", tuple.0, tuple.1, tuple.2);
    println!("tuple2: {:?}", tuple2);

    let (a, b, c) = tuple;  // destructuring
    println!("a: {}, b: {}, c: {}", a, b, c);

    /*
    functions
     */
    println!();
    // functions
    println!("2: {}", is_even(2));
}

fn is_even(num: u8) -> bool {
    let digit: u8 = num %2;
    digit == 0  // with out semicolon -> return statement
}