/**
 * Vectors example
 */
fn main() {
    // Create vector with known data types
    let mut vector1: Vec<i32> = Vec::new();
    let mut vector2: Vec<i32> = vec![]; // Using macro
    let mut vector3: Vec<i32> = vec![1i32, 2, 3]; // Add data type after 1st value

    let mut vector4 = vec![1, 2, 3];    // Compiler automatically knows the type
    let vector5: Vec<i32> = vec![1, 2, 3];  // Add data type
    let vector6: Vec<i32> = vec![0; 10];   // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    println!("{:?}", vector1);
    println!("{:?}", vector2);
    println!("{:?}", vector3);
    println!("{:?}", vector4);
    println!("{:?}", vector5);
    println!("{:?}", vector6);

    println!("-------------");

    vector1.push(0);
    println!("{:?}", vector1);
    println!("-------------");

    vector2.push(0);
    vector2.push(1);
    println!("{:?}", vector2);
    println!("-------------");

    vector3[0] = 3;
    println!("{:?}", vector3);
    println!("-------------");

    println!("{:?}", vector4);
    vector4.pop();
    println!("{:?}", vector4);
    println!("-------------");

    // Create a vector with initial capacity
    let mut vector7: Vec<i32> = Vec::with_capacity(5);
    println!("Length: {}, Capacity : {}", vector7.len(), vector7.capacity());

    vector7.push(0);
    vector7.push(1);
    vector7.push(2);
    vector7.push(3);
    vector7.push(4);
    println!("{:?}", vector7);
    println!("Length: {}, Capacity : {}", vector7.len(), vector7.capacity());

    vector7.push(5);
    println!("{:?}", vector7);
    println!("Length: {}, Capacity : {}", vector7.len(), vector7.capacity());
}