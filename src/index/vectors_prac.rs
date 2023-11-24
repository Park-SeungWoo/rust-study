pub fn vectors_fn() {  // vector -> dynamic array as we know
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("{}", vec.len());
    println!("{}", vec[0]);
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);
}