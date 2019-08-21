fn main() {
    // [a, b) range
    for i in 0 .. 5 {
        println!("Inside for: {}", i)
    }

    let mut i = 0;
    while i < 3 {
        println!("Inside while: {}", i);
        i += 1;
    }

    i = 0;

    loop {
        if i >= 3 {
            break;
        }
        
        println!("Inside loop: {}", i);
        i += 1;
    }
}
