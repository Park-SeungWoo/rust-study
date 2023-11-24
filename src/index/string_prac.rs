#![allow(unused)]

pub fn string_fn() {
    let str: &str = "hello world";  // Refers the string literal not the string object by making the object in heap memory.
    let mut str2: String = String::from("hello world");  // Make an object in heap memory and set the owner as the variable str2.

    let slice = &str2[.. 6];  // Borrowing without taking the ownership.
    println!("{}", slice.len());

    str2.push('1');
    str2.push_str("! Jeremy");
    str2 = str2.replace("hello", "bye");

    let str3: String = str2;  // move ownership

    println!("{}", str3);
}