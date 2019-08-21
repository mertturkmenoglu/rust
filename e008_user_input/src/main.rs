use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter a string: ");
    io::stdin().read_line(&mut input).ok().expect("Invalid input");

    println!("{}", input);
}
