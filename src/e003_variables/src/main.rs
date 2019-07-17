/**
 * Variable bindings, immutable / mutable types, constants, statics.
 */
fn main() {
    // Immutable by default
    let boolean_value = true;

    // You do not have to write type but you can
    let boolean_value2 : bool = true;

    // Multiple assignment
    let (int1, int2, int3) = (5, 12, 13);

    // You can make them mutable
    let mut var = 5;
    var = var + 1;

    // You do not have to specify the type for normal declarations
    // But when you define a const or a global variable, you have to add it to definition

    // You can define constants
    const LENGTH: i32 = 10;

    // Static keyword makes a global variable
    static GLOBAL_VALUE: i32 = 100;
}