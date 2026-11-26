// Immutable example
// fn main() {
//     let x: i32 = 5; // immutable variable means its value cannot be changed after initialization
//     println!("The value of x is: {}, again: {}", x, x); // {} is a placeholder for the value of x 
// }

// Mutable example
// fn main() {
//     let mut y: i32 = 10; // mutable variable means its value can be changed after initialization
//     println!("The value of y is: {}", y);
//     y = 20; // changing the value of y
//     println!("The new value of y is: {}", y);
// }

// Shadowing example
// fn main() {
//     let x: i32 = 5; // immutable variable
//     println!("The value of x is: {}", x);
//     let x: i32 = 11; // shadowing the previous x with a new x
//     println!("The new value of x is: {}", x);
// }

// Data types example
fn main() {
    
    // Scalar types in Rust include integers, floating-point numbers, booleans, and characters.
    let age: u32 = 18; // unsigned 32-bit integer (non negative)
    let num: i32 = -5; // signed 32-bit integer (can be negative)
    let temperature: f32 = 36.6; // 32-bit floating point number (can have decimal values)
    let is_active: bool = true; // boolean value (true or false)
    let grade: char = 'A'; // character value (single character)

    println!("age: {}, num: {}, temperature: {}, is_active: {}, grade: {}", age, num, temperature, is_active, grade);

    // Compound types in Rust include tuples and arrays.

    // tuples example
    let person: (&str, u32) = ( "Alice", 18); // tuple containing a string slice and an unsigned integer
    println!("Person: name: {}, age: {}", person.0, person.1);

    // arrays example
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array containing 5 integers
    println!("Numbers: {:?}", numbers);
}