pub fn structs_fn() {
    let name = String::from("bird");
    let bird: Bird = Bird {name: name, attack: 5};
    
    bird.print_name();
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