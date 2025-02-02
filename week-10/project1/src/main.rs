// Define a struct for Laptop
struct Laptop {
    brand: String,
    price: f64,
}

// Implement a method to calculate total cost
impl Laptop {
    fn calculate_total_cost(&self, quantity: u32) -> f64 {
        self.price * quantity as f64
    }
}

fn main() {
    // Create instances of Laptop
    let hp_laptop = Laptop {
        brand: "HP".to_string(),
        price: 650_000.0,
    };

    let ibm_laptop = Laptop {
        brand: "IBM".to_string(),
        price: 755_000.0,
    };

    let toshiba_laptop = Laptop {
        brand: "Toshiba".to_string(),
        price: 550_000.0,
    };

    let dell_laptop = Laptop {
        brand: "Dell".to_string(),
        price: 850_000.0,
    };

    // Calculate total cost for 3 laptops of each brand
    let quantity = 3;
    let total_cost_hp = hp_laptop.calculate_total_cost(quantity);
    let total_cost_ibm = ibm_laptop.calculate_total_cost(quantity);
    let total_cost_toshiba = toshiba_laptop.calculate_total_cost(quantity);
    let total_cost_dell = dell_laptop.calculate_total_cost(quantity);

    // Print total cost for each brand
    println!("Total cost for {} {} laptops: {:.2}", quantity, hp_laptop.brand, total_cost_hp);
    println!("Total cost for {} {} laptops: {:.2}", quantity, ibm_laptop.brand, total_cost_ibm);
    println!("Total cost for {} {} laptops: {:.2}", quantity, toshiba_laptop.brand, total_cost_toshiba);
    println!("Total cost for {} {} laptops: {:.2}", quantity, dell_laptop.brand, total_cost_dell);

    // Calculate overall total cost
    let overall_total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    // Print overall total cost
    println!("Overall total cost: {:.2}", overall_total_cost);
}