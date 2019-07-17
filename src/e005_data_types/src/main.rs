/**
 * Data types example
 */

fn main() {
    // Boolean
    let value1 = true;
    let value2: bool = true;

    println!("Value: {}", value1);
    println!("Value: {}", value2);

    // Character (Unicode)
    let value3 = 'a';
    let value4: char = 'ö';

    println!("Char: {}", value3);
    println!("Char: {}", value4);

    // Integer
    let integer1: i8 = 120;
    let integer2: i16 = 5000;
    let integer3: i32 = 1000000;
    let integer4: i64 = 1000000000;
    let integer5: i128 = -100;

    println!("i8   Value: {}\t\t\t\tMin: {}\t\t\t\t\tMax: {}", integer1, i8::min_value(), i8::max_value());
    println!("i16  Value: {}\t\t\tMin: {}\t\t\t\t\tMax: {}", integer2, i16::min_value(), i16::max_value());
    println!("i32  Value: {}\t\t\tMin: {}\t\t\t\tMax: {}", integer3, i32::min_value(), i32::max_value());
    println!("i64  Value: {}\t\t\tMin: {}\t\t\tMax: {}", integer4, i64::min_value(), i64::max_value());
    println!("i128 Value: {}\t\t\tMin: {}   Max: {}", integer5, i128::min_value(), i128::max_value());

    // Unsigned
    let unsigned1: u8 = 100;
    let unsigned2: u16 = 100;
    let unsigned3: u32 = 100;
    let unsigned4: u64 = 100;
    let unsigned5: u128 = 100;

    println!("u8   Value: {}\t\t\t\tMin: {} Max: {}", unsigned1, u8::min_value(), u8::max_value());
    println!("u16  Value: {}\t\t\t\tMin: {} Max: {}", unsigned2, u16::min_value(), u16::max_value());
    println!("u32  Value: {}\t\t\t\tMin: {} Max: {}", unsigned3, u32::min_value(), u32::max_value());
    println!("u64  Value: {}\t\t\t\tMin: {} Max: {}", unsigned4, u64::min_value(), u64::max_value());
    println!("u128 Value: {}\t\t\t\tMin: {} Max: {}", unsigned5, u128::min_value(), u128::max_value());

    // Float
    let float1: f32 = 3.14; // Avoid using f32
    let float2: f64 = std::f64::consts::PI;

    println!("f32: {}", float1);
    println!("f64: {}", float2);

    // Arrays
    // They are immutable by default. Their elements can not be changed even they are defined as mut

    let array1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut array2 = [1, 3, 5, 7, 9];

    // Empty array
    let array3: [i32; 0] = [];

    // Another definition
    let array4: [i32; 5] = [0, 0, 0, 0 , 0];

    // Another way to init
    let array5 = [0; 5]; // [0, 0, 0, 0, 0]

    println!("{:?}", array1);
    println!("{:?}", array2);
    println!("{:?}", array3);
    println!("{:?}", array4);
    println!("{:?}", array5);

    // Tuples
    let tuple1 = (0, 3.14, true, 'm', "String");
    let tuple2: (i32, f64, &str) = (0, 3.14, "Rust");

    let (var1, var2, var3) = tuple2;
    let (var4, _, _, _, var5) = tuple1;

    let tuple3 = ("First element", );
    let tuple4 = (tuple1, ("my", "tuple"), 5);

    println!("{:?}", tuple1);
    println!("{:?}", tuple2);
    println!("{:?}", tuple3);
    println!("{:?}", tuple4);

    // Slices
    let main_array: [i32; 5] = [0, 1, 2, 3, 4];
    let slice1: &[i32] = &main_array;
    let slice2 = &main_array[0..5];
    let slice3 = &main_array[..];

    let slice4 = &main_array[1..3];
    let slice5 = &main_array[3..];
    let slice6 = &main_array[..4];

    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);
    println!("{:?}", slice4);
    println!("{:?}", slice5);
    println!("{:?}", slice6);

    // String
    let str1 = "String";
    let str2: &str = "Another String";

    let str3: String = str1.to_string();

}