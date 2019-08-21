use std::collections::HashSet;

fn main() {
    let mut hs = HashSet::new();
    hs.insert("emily");
    hs.insert("diana");
    hs.insert("barbara");
    hs.insert("selina");

    hs.remove("diana");

    if hs.contains("diana") {
        println!("diana is in the set");
    } else {
        println!("diana is not in the set");
    }

    if hs.contains("emily") {
        println!("emily is in the set");
    } else {
        println!("emily is not in the set");
    }

    for e in &hs {
        println!("{}", e);
    }
}
