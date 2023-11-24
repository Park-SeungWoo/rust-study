pub fn enums_fn() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(10);
    let c: MyEnum = MyEnum::C { x: 20, y: 20 };
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b {  // Extract values inside the enum
        println!("{}", val);
    }

    if let MyEnum::C { x, y } = c {
        println!("{}, {}", x, y);
    }
}

#[derive(Debug)]  // Let the compiler know about how to print our enum (In my opinion, it tells that our enum is derived from Debug)
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}