fn main() {
    let arr1: [i32; 4] = [1, 2, 3, 5];
    println!("Array: {:?}", arr1);

    println!("========");

    // Iterate over array
    for i in 0 .. arr1.len() {
        println!("Index: {}\tElement: {}", i, arr1[i]);
    }

    println!("========");

    // Like a for each loop
    for e in arr1.iter() {
        println!("Element: {}", e);
    }

    println!("========");

    // Pass by value
    print_arr(arr1);
    println!("========");

    // Create mutable array.
    let mut arr2 = [0, 0, 0, 0];
    print_arr(arr2);
    
    println!("========");

    // Pass by reference
    change_arr(&mut arr2);
    print_arr(arr2);
}

fn print_arr(arr: [i32; 4]) {
    for e in arr.iter() {
        println!("{}", e)
    }
}

fn change_arr(arr: &mut [i32; 4]) {
    for i in 0 .. 4 {
        arr[i] = 1;
    }
}