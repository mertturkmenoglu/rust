fn main() {
    // It is like nullable type in Kotlin
    let mut name: Option<String> = None;
    name = Some("Emily".to_string());

    match name {
        Some(name) => println!("Name: {}", name),
        None => println!("Unknown")
    }
}
