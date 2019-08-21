fn main() {
    // String literal
    let str_literal: &str = "This is a string literal";

    // String object
    let str_obj1 = String::from("This is a string object");
    let str_obj2 = "This is a string object(2)".to_string();

    println!("String literal: {}", str_literal);
    println!("String object1: {}", str_obj1);
    println!("String object2: {}", str_obj2);

    // Split whitespace
    let message = String::from("Emily is the best");
    for word in message.split_whitespace() {
        println!("{}", word)
    }

    // Split comma
    let names: Vec<&str> = "Emily,Diana,Barbara".split(",").collect();
    for name in names {
        print!("{}\t", name)
    }

    println!();

    // String concat
    let name = String::from("Emily ");
    let surname = String::from("Smith");
    let full_name = name + &surname;
    println!("{}", full_name);

    // To string
    let age = 35;
    let age_str = age.to_string();
    println!("{}", age_str);
}
