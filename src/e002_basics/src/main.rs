/**
 * Formatted output, macros and basic variable declaration
 */
fn main() {
    // Classic string
    println!("Rust is awesome");

    // Replace
    println!("{} {}", "Rust", "is awesome");

    // You can specify the order
    println!("{0} {2} {1}", "Rust", "awesome", "is");

    // Just like in Python
    println!("{lang} {verb} {adj}", verb="is", lang="Rust", adj="awesome");

    println!("{:?}", [1, 2, 3]);
    println!("{:#?}", [1, 2, 3]);

    let message = format!("{} {} {}", "Rust", "is", "awesome");
    println!("{}", message);
}