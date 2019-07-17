/**
 * Rust functions
 */
fn add(first: i32, second: i32) -> i32 {
    return first + second;
}

fn main() {
    let a: i32;
    a = add(3, 5);
    println!("{}", a);

    // Function pointers
    let fp1 = add;
    let b = fp1(1, 2);

    // You can add the type
    let fp2: fn(i32, i32) -> i32 = add;
    let c = fp2(2, 3);

    // Anonymous functions
    let print_x = |x: i32| {
        println!("{}", x);
    };

    print_x(b);
    print_x(c);
}