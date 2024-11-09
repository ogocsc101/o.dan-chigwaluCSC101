use std::io;
fn main() {
    //input values for a, b and c
    let mut input = String::new();
    println!("Enter value for a: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a:f64 = input.trim().parse().expect("please enetr a valid number");

    input.clear();
    println!("Enter a value for b:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b:f64 = input.trim().parse().expect("Please enter a valid value");

    input.clear();
    println!("Enter value for c: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c:f64 = input.trim().parse().expect("Please enter a valid value");
    //calculation for discriminant
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        println!("Two real roots: {} and {}",
               (-b + discriminant.sqrt()) / (2.0 * a),
               (-b - discriminant.sqrt()) / (2.0 * a));

    }else if discriminant == 0.0 {
        println!("One real root: {}", -b / (2.0 * a));
    }else {
        println!("No real roots");
    }

}
