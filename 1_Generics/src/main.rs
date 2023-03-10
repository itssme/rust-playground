struct Animal {
    animal_type: String,
}

trait MakeNoise {
    fn make_noise(&self) -> String;
}

impl MakeNoise for Animal {
    fn make_noise(&self) -> String {
        let noise = match self.animal_type.as_str() {
            "Cat" => { "miau" }
            "Dog" => { "wuff" }
            _ => { "**silence**" }
        };
        String::from(noise)
    }
}

struct HelloStruct<T> {
    name: String,
    age: i8,
    animal: T,
}

trait Hello {
    fn hello(&self);
}

impl<T: MakeNoise> Hello for HelloStruct<T> {
    fn hello(&self) {
        println!("Hello {}, you are {} years old! Your animal makes a: {}", self.name, self.age, self.animal.make_noise())
    }
}


fn main() {
    let joel = HelloStruct { name: String::from("Joel"), age: 100, animal: Animal { animal_type: String::from("Cat") } };
    joel.hello();

    let kajetan = HelloStruct { name: String::from("Kajetan"), age: 100, animal: Animal { animal_type: String::from("Dog") } };
    kajetan.hello();
}
