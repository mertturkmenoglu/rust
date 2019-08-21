struct Person {
    name: String,
    age: i16
}

fn main() {
    let p = Person {name: "Emily".to_string(), age: 35};
    println!("Name: {}\tAge: {}", p.name, p.age);
    print_formatted(&p);
    let p2 = get_person();
    print_formatted(&p2);
}

fn print_formatted(p: &Person) {
    println!("Person --> Name: {}\tAge: {}", p.name, p.age);
}

fn get_person() -> Person {
    return Person { name: "Diana".to_string(), age: 21 };
}
