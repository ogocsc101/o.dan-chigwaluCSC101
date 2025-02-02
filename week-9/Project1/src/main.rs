use std::fs::write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

   
    let mut content = String::from("Lager:\n");

    for drink in &lager {
        content += format!("{}\n", drink).as_str();
    }

    content += "\nStout\n";
    for drink in &stout {
        content += format!("{}\n", drink).as_str();
    }

    content += "\nNon-Alcoholic\n";
    for drink in &non_alcoholic {
        content += format!("{}\n", drink).as_str();
    }

    write("drinks.txt", content).unwrap();
    println!("Drinks categories saved to 'drinks.txt'.");
}
