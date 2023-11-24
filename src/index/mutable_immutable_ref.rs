pub fn mut_immut_fn() {
    let mut arr: [u8; 5] = [1, 2, 3, 4, 5];
    let mut arr2 = arr;  // Copy values because it's in stack memory
    arr2[0] = 10;

    println!("{:?}", arr);
    
    let arr4 = &arr;  // immutable borrowing
    let arr5 = &arr;  // It's okay we borrow two times or more immutably.
    // let mut arr3 = &arr[0..=arr.len() - 1];  // slicing immutably
    // let arr3 = &mut arr[0..=3];  // slicing mutably
    let arr3 = &mut arr;
    arr3[1] = 9;

    
    // println!("{:?}, {:?}", arr4, arr5);
    
    // let arr6 = &arr;  // err because arr is already mutably borrowed
    // let arr7 = &mut arr;  // we can not mutably borrow twice or more in the same scope
    
    println!("{:?}, {:?}", arr2, arr3);

    println!("{:?}", arr);  // Same as arr3, because arr3 mutably borrowed arr and it modified the value in index 1 as 9.
    
    /*
    It's about mutable, immutable borrow.
    There're some rules to use borrow statement.
    1. If we already borrow some value as immutable, we cannot mutably borrow it again.
    2. If we already borrow some value as mutable, we cannot immutably borrow it again.
    These two conditions are applied when the variable that mut or immutably borrowed some value for the first time, is used after the second borrow statement.
    3. We can not mutably borrow some value which is already mutably borrowed.
    It doesn't matter if we immutably borrow some value again and again, While mutable references are not allowed multiple statement in the same scope.
     */
}