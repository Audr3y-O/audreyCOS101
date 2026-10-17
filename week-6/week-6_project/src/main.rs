use std::io;

fn main() {
    println!("=========== THE RESTAURANT MENU ===========");
    println!("P  Poundo Yam / Edinkaiko Soup   N3,200");
    println!("F  Fried Rice & Chicken          N3,000");
    println!("A  Amala & Ewedu Soup            N2,500");
    println!("E  Eba & Egusi Soup              N2,000");
    println!("W  White Rice & Stew             N2,500");
    println!("===========================================");

    // Read the food type
    println!("Enter food type (P, F, A, E or W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase();

    // Read the quantity
    println!("Enter quantity:");
    let mut qty_text = String::new();
    io::stdin().read_line(&mut qty_text).expect("Failed to read input");
    let quantity: f64 = qty_text.trim().parse().expect("Please enter a number");

    // Decide the price from the letter
    let price: f64;
    if food == "P" {
        price = 3200.0;
    } else if food == "F" {
        price = 3000.0;
    } else if food == "A" {
        price = 2500.0;
    } else if food == "E" {
        price = 2000.0;
    } else if food == "W" {
        price = 2500.0;
    } else {
        println!("Invalid food type.");
        return;
    }

    // Compute the total
    let mut total = price * quantity;
    println!("Subtotal: N{}", total);

    // Last if: 5% discount when total is greater than N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("Discount (5%): N{}", discount);
    }

    println!("Total to pay: N{}", total);
}