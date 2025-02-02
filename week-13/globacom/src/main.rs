use std::io::Read;
use std::io;

fn staff_table(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn database(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn department_table(){ 
    let mut file = std::fs::File::open("department_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn project_table(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);

}

fn dataplan_table(){
     let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn customer_table(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn main() {
    print!("Please enter your role: ");
    println!("A- Administrator,
              P- project_manager, 
              E- employee,
              C- customer,
              V- vendor");
    let mut user_role = String::new();
    io::stdin().read_line(&mut user_role).expect("Failed to read line");
    let user_role = user_role.trim(); 

    if user_role == "A" {
        println!("This is the Database");
        database();
    }else if user_role == "P" {
        println!("This is the project table");
        project_table();
    }else if user_role == "E" {
        println!("This is the staff table");
    staff_table();
    }else if user_role == "C"{
        println!("This is the customer table");
customer_table();
    }else if user_role == "V"{ 
        println!("This is the dataplan table");
        dataplan_table();
    } else {
        println!("Please enter a valid role");
    }
}
