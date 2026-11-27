// Iterate and Closure
// fn main() {
//     let price = vec![2.00, 12.00, 3.12, 6.213, 8.010, 1.1200];
//     // Calculate the total of prices over $5.00
//     let total: f64 = price
//         .iter() // Convert the vector into an iterator
//         .filter(|p: &&f64| **p > 5.00) // Keep only prices over $5.00
//         .fold(0.0, |acc, p| acc + p); // Sum all prices over $5.00

//     println!("Total over $5.00 is: ${}", total);

//     let formatted: Vec<String> = price.iter().map(|p| format!("${:.2}", p)).collect();

//     println!("Formatted prices: {:?}", formatted);

//     // Closures

//     // Readonly closure example
//     // The closure will capture the label variable from the surrounding scope
//     let label = String::from("Price List");
//     // Define a closure that prints the label
//     let print_label = || println!("{}", label);

//     // Call the closure multiple times to print the label
//     print_label();
//     print_label();
//     print_label();

//     // Write closure example

//     let mut counter = 0;

//     let mut increment_counter = || {
//         counter += 1;
//         println!("Counter incremented to: {}", counter)
//     };

//     // Call the write closure multiple times to increment the counter
//     increment_counter();
//     increment_counter();
//     increment_counter();

//     println!("Counter value: {}", counter);

//     // Own the value closure example

//     let data = String::from("Some important data");

//     let consume = move || data;

//     let recovered = consume();

//     println!("recovered data; {}", &recovered);
//     println!("recovered data; {}", recovered);
// }

// Lifetime Example
// fn main() {
//     let results = longest("rust", "python");
//     println!("results is {}", results);

//     let owner = String::from("Config Owner");

//     let cfg = Config { name: &owner };

//     println!("config owner name is {}", cfg.name);
// }

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() { a } else { b }
// }

// struct Config<'a> {
//     name: &'a str,
// }

// // Invalid example of lifetime

// fn broken_example() {
//     let cfg: Config;
//     {
//         let owner = String::from("New Owner");
//         cfg = Config { name: &owner };
//         println!("owner in internal scope is {}", &owner);
//     }
//     println!("cfg owner in outer scope is {}", cfg.name)
// }

// Smart Pointers Box, RC, RefCell
// Import common library for smart pointers
// use std::{cell::RefCell, rc::Rc};

// // Define a simple linked list using Box smart pointer for heap allocation
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn main() {
//     let list = List::Cons(
//         1,
//         Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
//     );

//     let mut current = &list;

//     loop {
//         match current {
//             List::Cons(value, next) => {
//                 println!("list value {}", value);
//                 current = next;
//             }
//             List::Nil => break,
//         }
//     }

//     // Shared ownership with Rc example
//     let name = Rc::new(String::from("config master"));
//     let name2 = Rc::clone(&name);

//     println!(
//         "number of references name variable have: {}",
//         Rc::strong_count(&name)
//     );

//     println!("name2 : {}", name2);

//     // Interior mutability with RefCell example
//     let count = RefCell::new(0);
//     *count.borrow_mut() += 1;

//     println!("count value {}", count.borrow());

//     // Shared ownership with Rc and interior mutability with RefCell
//     let shared = Rc::new(RefCell::new(Vec::<String>::new()));
//     shared.borrow_mut().push("item".to_string());

//     println!("{:?}", shared.borrow())
// }

// Advance Traits

fn main() {
    println!("Advanced traits example");
}
