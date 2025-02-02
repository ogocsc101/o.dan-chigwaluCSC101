use std::io::{self, Write};

fn main() {
    // Define food items and their prices
    let menu = [
        ("Pounded Yam/Edinkaiko Soup", 3200),
        ("Fried Rice & Chicken", 3000),
        ("Amala & Ewedu Soup", 2500),
        ("Eba & Egusi Soup", 2000),
        ("White Rice & Stew", 2500),
    ];

    // Display menu
    println!("Menu:");
    for (index, (food, price)) in menu.iter().enumerate() {
        println!("{}. {} - N{}", index + 1, food, price);
    }

    // Ask for the food choice and quantity
    let mut total_cost = 0;

    loop {
        print!("\nEnter the food number you want to order (or '0' to finish): ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut food_choice = String::new();
        io::stdin().read_line(&mut food_choice).expect("Failed to read line");
        let food_choice: usize = food_choice.trim().parse().unwrap_or(0);

        if food_choice == 0 {
            break; // Exit the loop if the user enters 0 (finish the order)
        }

        // Check if the choice is valid
        if food_choice < 1 || food_choice > menu.len() {
            println!("Invalid choice. Please enter a number between 1 and {}.", menu.len());
            continue;
        }

        // Ask for the quantity of the chosen food
        print!("Enter the quantity: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = quantity.trim().parse().unwrap_or(0);

        // Get the price for the chosen food item
        let price_per_item = menu[food_choice - 1].1;

        // Calculate the cost for the current item and add it to total
        total_cost += (price_per_item * quantity) as u32;
    }

    // Display the total before any discount
    println!("\nTotal cost: N{}", total_cost);

    // Apply discount if total cost exceeds N10,000
    if total_cost > 10000 {
        let discount = total_cost as f32 * 0.05;
        let discounted_total = total_cost as f32 - discount;

        println!("A 5% discount has been applied.");
        println!("Discounted total cost: N{:.2}", discounted_total);
    } else {
        println!("No discount applied.");
    }
}
