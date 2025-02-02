use std::io;

fn main() {
    println!("What operation do you want to perform?");
    println!("Enter :
        1 for Area of trapezim,
        2 for Area of rhombus,
        3 for Area of parallelogram,
        4 for Area of cube,
        5 for Volume of cylinder.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let operation:u32 = input.trim().parse().expect("Not a valid number");
if operation==1 {
     trapezium();
 }else if operation==2 {
 rhombus(); }
 else if operation==3 {parallelogram();}
 else if operation==4{cube();}
 else if operation==5 {
cylinder() }
else {
    println!("Sorry, This is not a valid function." );
}
}

fn trapezium(){ 
    println!("Enter the height");
    let mut heights = String::new();
    io::stdin().read_line(&mut heights).expect("Failed to read input");
    let height:i32 = heights.trim().parse().expect("Failed to input");
    println!("Enter the first base");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let a:i32 = base1.trim().parse().expect("Failed to input");
    println!("Enter the second base");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let b:i32 =  base2.trim().parse().expect("Failed to input");

    let area_of_trapezium = (height / 2)*(a + b);
    println!("The area of the trapezim is {}",area_of_trapezium );
}

fn rhombus(){
    let mut diagonal1 = String::new();
    println!("Enter the first diagonal");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let a:i32 =  diagonal1.trim().parse().expect("Failed to input");
let mut diagonal2 = String::new();
println!("Enter the second diagonal");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let b:i32 =  diagonal2.trim().parse().expect("Failed to input");
let area_of_rhombus = (a * b) / 2;
println!("The area of the rhombus is {}",area_of_rhombus );
}
fn parallelogram(){
    println!("Enter the base");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let a:i32 = base1.trim().parse().expect("Failed to input");
    println!("Enter the height");
    let mut heights = String::new();
    io::stdin().read_line(&mut heights).expect("Failed to read input");
    let height:i32 = heights.trim().parse().expect("Failed to input");
    let area_of_parallelogram = a * height;
println!("The area of the parallelogram is {}",area_of_parallelogram );

}
fn cube(){
    let mut lenght = String::new();
     println!("Enter the lenght of the cube");
io::stdin().read_line(&mut lenght).expect("Failed to read input");
    let a:i32 = lenght.trim().parse().expect("Failed to input");
    let area_of_cube = 6* a.pow(2);
println!("The area of the cube is {}",area_of_cube );
}
fn cylinder(){
    let pie = 22/7;
    let mut heights = String::new();
    println!("Enter the height of the cylinder");
    io::stdin().read_line(&mut heights).expect("Failed to read input");
    let height:i32 = heights.trim().parse().expect("Failed to input");
    let mut radius = String::new();
     println!("Enter the radius of the cylinder");
io::stdin().read_line(&mut radius).expect("Failed to read input");
    let a:i32 = radius.trim().parse().expect("Failed to input");
    let volume_of_cylinder = pie*a.pow(2) * height;
println!("The volume of the cylinder is {}", volume_of_cylinder);

}