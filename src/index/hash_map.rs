use std::collections::HashMap;

pub fn hash_map_fn() {
    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "hi2");
    println!("{:?}", map);

    let test: &str;
    match map.get(&0) {  // Must pass the address of the key
         Some(str) => {
            println!("{}", str);
            test = str;
        },
         None => {
            println!("none");
            test = "none";
        }
    }
    println!("{}", test);

    match map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map")
    }

    map.remove(&0);
    println!("{:?}", map);
}