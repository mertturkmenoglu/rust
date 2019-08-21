fn add_one(n: &mut i32) {
    *n = 1;
}

fn main() {
    let mut number = 42;
    println!("Number: {}", number);

    add_one(&mut number);
    println!("Number: {}", number);
}
