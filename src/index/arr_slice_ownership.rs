pub fn arr_slice_fn() {
    /*
    Every types i.e primitive type that has pre-declaration of size is stored in stack memory
    And also every values in rust has owner.
    The ownership has difference between stack, heap
    In stack, the value will be copied typically. Imagine the behavior in c-lang about the stack-stored value.
    In heap, the value can have only one owner. So, it can be referenced by the only one variables.
    And there are some concepts related with ownership called move, borrowing.
    Move means the modification of any value's owner stored in heap memory.
    Borrowing allows the reference without modifying the value's owner. i.e. just reference without having ownership
    In borrowing statement, we can use the value but we can not modify the value.
    If you want to, then use Mutable references keyword (let arr2 = &mut arr).
    And there is a terminology called Slice. It's also a type of borrowing.
     */
    let arr: [u8; 6] = [0, 1, 2, 3, 4, 5];  // In stack, also arr is a pointer
    let sliced = &arr[1 .. 3];  // [inclusive .. exclusive] like python slicing [idx:idx]

    borrowing_slice(arr, sliced);

    let arr2 = arr;  // Copy the value in stack memory. 
    println!("{:?}", arr2);


    // ownership, move, borrowing
    /*
    It refers to the object which is owned by str variable (in heap), 
    And also the string object has owner variable which means literally its owner, 
    It'll be removed when the owner exits the scope e.g {} or lifetime. (Rust compiler will call drop() implicitly to remove obj in heap)
    */
    let str = String::from("hello");  
    /*
    Change the owner to arr2. 
    And it's called as "move" 
    Rust never deep copy the data implicitly, 
    Without move, str and st2 will remove same object in heap when the scope is finished. and it could contaminate the memory. Also it'll make the program vulnerable to attacks.
     */
    let str2 = str;
    // println!("{}", str);  // err -> borrow of owner-moved value
    /*
    To deep copy the object, use clone() method
     */
    let str3 = str2.clone();
    println!("{} {}", str2, str3);

    let hello: String = String::from("hello");
    let world: String = String::from("world!");

    borrowing_str(hello, &world);
    // println!("{}, {}", hello, world);  // err because the ownership of the value have taken by hello variable was moved to the parameter named str in function borrowing_str()


}

fn borrowing_slice(arr: [u8; 6], slice: &[u8]) {  // borrowing slice -> with ampersand, it allows refer to some value without taking ownership.
    /*
    While we don't know the length of 'slice' array, we can borrow it and use.
     */
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("slice length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);

    // slice[0] = 10;  // err because 
}

fn borrowing_str(str: String, str2: &String) {  // borrowing_str(move, borrow) -> With ampersand, It allows refer to some value without taking ownership. And without it, it moves the ownership to this function's scope.
    println!("{}, {}", str, str2);
}

/*
Refer to these sites for more information
https://dgkim5360.tistory.com/entry/what-i-learned-from-the-rust-book-chapter-4-1-what-is-ownership
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    */