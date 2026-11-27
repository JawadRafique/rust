mod models;

use models::Product;

fn main() {
    let product = Product {
        name: "Iphone".to_string(),
        price: 999.99,
    };

    println!("Product: {}, Price: {}", product.name, product.price);
}
