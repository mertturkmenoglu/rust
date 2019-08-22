fn main() {
    let name = String::from("Emily");
    let surname = String::from("Smith");
    let mut msg = "Hello".to_owned() + " " + &name + " " + &surname;
    println!("Original: {}", msg);

    msg.push('.');
    println!("Push: {}", msg);

    println!("Upper case: {}", msg.to_uppercase());
    println!("Lower case: {}", msg.to_lowercase());

    // Format
    let name = "Emily";
    let age = 35;
    let msg2: String = format!("{} is {} years old", name, age);
    println!("Formatted: {}", msg2);

    // Char at
    let ch = name.chars().nth(3).unwrap();
    println!("Char at 3: {}", ch);

    let long_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris turpis orci...";

    // Substr
    let sub = &long_str.chars().skip(10).take(11).collect::<String>();
    let sub2 = &long_str[10 .. 21];
    println!("sub: {}", sub);
    println!("sub2: {}", sub2);

    // Replace
    let new_str = str::replace(sub, "dolor", "DOLOR");
    println!("Replace: {}", new_str);

    // Find
    if let Some(found) = sub.find("sit") {
        println!("Found at: {}", found);
    }
}
