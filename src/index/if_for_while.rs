pub fn if_for_while_fn() {
    let mut n = 1;
    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }

    for i in 0..6 {  // It's pythonic, 0..6 will make a range(0, 6)
        println!("{}", i);
    }

    while n < 6 {
        if n == 4 {
            println!("exit");
            break;
        }
        println!("{}", n);
        n += 1;
    };

    let mut t = 10;
    let test = loop {
        if t < 5 {
            break t;
        }
        t -= 1;
    };
    println!("{}", test);

    let i = 4;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1, 2"),
        3..=4 => println!("3, 4"),  // Typically, 3..4 makes the range of 3 to 3 i.e. just 3. But 3..=4 makes the exclusive element at behind inclusive.
        _ => println!("default")
    }
}