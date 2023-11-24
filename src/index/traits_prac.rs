pub fn traits_fn() -> () {  // return void
    let bird = Bird {name: String::from("bird"), attack: 5};
    println!("{}", bird.can_fly());
    println!("{}", bird.is_animal());
}

struct Bird {
    name: String,
    attack: u64
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}
/*
Rust doesn't supports inheritance. It just supports interfaces.
*/
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }

    fn is_animal(&self) -> bool {
        println!("It's Bird");
        true
    }
}