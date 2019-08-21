use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    
    m.insert("emily", 35);
    m.insert("diana", 42);
    m.insert("barbara", 21);
    m.insert("selina", 37);

    if let Some(age) = m.get("emily") {
        println!("emily: {}", age);
    }

    m.remove("barbara");

    for (k, v) in &m {
        println!("Key: {} - Value: {}", k, v);
    }
}
