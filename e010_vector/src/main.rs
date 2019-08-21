fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for i in 0 .. 5 {
        v1.push(i);
    }

    for e in &v1 {
        v2.push(e + 1);
    }

    for (i, e) in v1.iter().enumerate() {
        println!("Index: {}\tElement: {}", i, e);
    }

    for (x, y) in v1.iter().zip(v2.iter()) {
        println!("First: {}\tSecond: {}", x, y);
    }
}
