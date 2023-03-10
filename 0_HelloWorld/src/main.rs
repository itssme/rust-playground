struct HelloStruct {
    name: String,
    age: i8,
}

trait Hello {
    fn hello(&self);
}

impl Hello for HelloStruct {
    fn hello(&self) {
        println!("Hello {}, you are {} years old!", self.name, self.age)
    }
}

fn print_stuff(name: String) {
    println!("Hello {}", name)
}

fn main() {
    println!("Hello, world!");
    print_stuff(String::from("itssme"));

    let itssme = HelloStruct { name: String::from("itssme"), age: 100 };
    itssme.hello();
}
