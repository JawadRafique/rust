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
// fn main() {
    
//     // Scalar types in Rust include integers, floating-point numbers, booleans, and characters.
//     let age: u32 = 18; // unsigned 32-bit integer (non negative)
//     let num: i32 = -5; // signed 32-bit integer (can be negative)
//     let temperature: f32 = 36.6; // 32-bit floating point number (can have decimal values)
//     let is_active: bool = true; // boolean value (true or false)
//     let grade: char = 'A'; // character value (single character)

//     println!("age: {}, num: {}, temperature: {}, is_active: {}, grade: {}", age, num, temperature, is_active, grade);

//     // Compound types in Rust include tuples and arrays.

//     // tuples example
//     let person: (&str, u32) = ( "Alice", 18); // tuple containing a string slice and an unsigned integer
//     println!("Person: name: {}, age: {}", person.0, person.1);

//     // arrays example
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array containing 5 integers
//     println!("Numbers: {:?}", numbers);
// }

// Multi Function example
// fn main() {
//     greeting("Alice");
//     println!("The result of addition is: {}", add(32, 10));
// }

// fn greeting(name: &str) {
//     println!("Hello and welcome to the community, {}!", name);
// }

// fn add (a: i32, b: i32) -> i32 {
//     a + b // any line without a semicolon is treated as the return value of the function
// }

// if else statement
// fn main () {
//     let number: i32 = -10;

//     if number > 0 {
//         println!("The number is positive");
//     } else if number < 0 {
//         println!("The number is negative");
//     } else {
//         println!("The number is zero");
//     }

//     let label: &str = if number > 0 {"positive"} else if number < 0 {"negative"} else {"zero"};
//     println!("The label is: {}", label);
// }

// Loop
// fn main () {
//     for i in 1..4 {
//         println!("i: {}", i);
//     }

//     let scores = [90, 85, 78, 92, 88]; // array of scores

//     for score in scores {
//         println!("Score: {}", score);
//     }
// }

// Ownership
// fn main () {
//     let s1 = String::from("hello"); // s1 comes into scope
//     let s2 = s1 + " world"; // s1 is moved to s2, s1 is no longer valid
//     println!("s2: {}", s2); 

//     // cloning s2 to create a new String s3
//     let s3 = s2.clone() + "!";
//     println!("s3: {}", s3);
// }

// References and Borrowing
// fn main () {
//     let s1 = String::from("hello"); // s1 comes into scope
//     let s2 = &s1; // s1 is borrowed by s2, s1 is still valid
//     println!("s1: {}, s2: {}", s1, s2); 

//     let len = calculate_length(&s1);
//     println!("The length of s1 is: {}", len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example of mutable references and borrowing rules in Rust
// This will cause a compile-time error because Rust only allows one mutable reference to a particular piece of data in a particular scope at a time.
// fn main () {
//     let mut s1 = String::from("hello"); // s1 comes into scope

//     let r1: &mut String = &mut s1;
//     let r2: &mut String = &mut s1;

//     print!("r1: {}, r2: {}", r1, r2);
// }

// fn main () {
//     let mut s1 = String::from("hello"); // s1 comes into scope

//     add_exclamation(&mut s1);
//     println!("s1 after adding exclamation: {}", s1);
// }

// fn add_exclamation(s: &mut String) {
//     s.push('!')
// }

// Structs in Rust
// struct User {
//     username: String,
//     age: u32,
//     active: bool,
// }
// Main function demonstrating the usage of the User struct and its methods
// fn main () {
//     let user1 = User {
//         username: String::from("john_doe"),
//         age: 30,
//         active: true,
//     };
//     // println!("Username: {}, Age: {}, Active: {}", user1.username, user1.age, user1.active); 
//     println!("{}", user1.describe());
// }

// Implementation of the User struct methods
// impl User{
//     // Method to describe the user as a string
//     fn describe(&self) -> String {
//         // Using the format! macro to create a descriptive string for the user
//         format!("Username: {}, Age: {}, Active: {}", self.username, self.age, self.active)
//     }
// }

// Enums 
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// enum Shapes {
//     Circle(f64),
//     Rectangle(f64, f64),
//     Triangle(f64, f64),
// }

// fn area(shape: &Shapes) -> f64 {
//     match shape {
//         Shapes::Circle(radius) => 3.14 * radius * radius,
//         Shapes::Rectangle(width, height) => width * height,
//         _ => 0.0 // Default case for any other shapes not handled above
//     }
// }

// fn main() {
//     // Initialize the traffic light to Red
//     let light = TrafficLight::Red;

//     // Initialize shapes
//     let c = Shapes::Circle(5.0);
//     let r = Shapes::Rectangle(3.0, 4.0);
//     let t = Shapes::Triangle(3.0, 4.0);

//     // Match on the traffic light and print the corresponding action
//     match light {
//         TrafficLight::Red => println!("Stop!"),
//         TrafficLight::Yellow => println!("Caution!"),
//         TrafficLight::Green => println!("Go!"),
//     }
    
//     println!("Area of the circle: {}", area(&c));
//     println!("Area of the rectangle: {}", area(&r));
//     println!("Area of the triangle: {}", area(&t));
// }

// Option type in Rust represents an optional value: every Option is either Some and contains a value, or None, and does not.
// Example usage of Option type
// fn find_user(id: u32) -> Option<String> {
//     if id == 1 {
//         Some(String::from("John Doe"))
//     } else {
//         None
//     }
// }
// fn main() {
//     let results = find_user(1);
//     match results {
//         Some(name) => println!("Found user: {}", name),
//         None => println!("User not found"),
//     }
// }

// Result type in Rust represents either success (Ok) or failure (Err).
// Example usage of Result type
// fn divide(a: f64, b: f64) -> Result<f64, String> { // Returns Ok(result) if division is successful, Err(message) if division by zero 
//     if b == 0.0 {
//         Err(String::from("Cannot divide by zero"))
//     } else {
//         Ok(a / b)
//     }
// }

// fn main() {
//     match divide(10.0, 2.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     } 

//     match divide(10.0, 0.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// Example of Vector in Rust
fn main() {
    let mut scores: Vec<i32> = Vec::new();

    scores.push(10);
    scores.push(20);
    scores.push(30);
    
    for score in &scores {
        println!("Score: {}", score);
    }

    println!("First score count: {}", scores[0]);
    
    // Demonstrates ownership and borrowing in Rust
    // `owned` is a String that owns its data
    // `borrowed` is a string slice that borrows data from a string literal
    let owned: String = String::from("Hello, Rust!");
    // &str for read-only access to string data
    let borrowed: &str = "world";

    println!("Owned: {}", owned);
    println!("Borrowed: {}", borrowed);
}

