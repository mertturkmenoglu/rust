struct Person {
    name: String,
    age: i32
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self { name: name.to_string(), age: age }
    }

    fn print_info(&self) {
        println!("Person --> Name: {} - Age: {}", self.name, self.age);
    }
}


fn main() {
    let e = Person::new("Emily", 35);
    e.print_info();
}
