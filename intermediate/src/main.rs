fn main() {
    let price = vec![2.00, 12.00, 3.12, 6.213, 8.010, 1.1200];
    // Calculate the total of prices over $5.00
    let total: f64 = price
        .iter() // Convert the vector into an iterator
        .filter(|p: &&f64| **p > 5.00) // Keep only prices over $5.00
        .fold(0.0, |acc, p| acc + p); // Sum all prices over $5.00 

    println!("Total over $5.00 is: ${}", total);

    let formatted: Vec<String> = price.iter().map(|p| format!("${:.2}", p)).collect();

    println!("Formatted prices: {:?}", formatted);
}
