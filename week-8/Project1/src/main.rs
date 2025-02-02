use std::io;
fn main(){let mut input1 = String::new();
let mut input2 = String::new();

println!("T for teacher\nL for lawyer\nA for academic\nO for office administrator");
io::stdin().read_line(&mut input1).expect("failed to input ");
let profession: &str = input1.trim();

println! ("How many years of experience do you have? ");
io::stdin().read_line(&mut input2).expect("failed to read input ");
let years_of_experience:f32 = input2.trim().parse().expect("failed to input ");

let office_admin_vector = vec! [
"Intern ",
"Administrator ",
"Office Manager ",
"Senior Administrator ",
"Director ",
"CEO ",
];

let lawyer = vec![

"Paralegal ",
"Junior associate ",
"Associate ",
"Senior associate 1-2 ",
"Senior associate 3-4 ",
"Partner ",
];

let academic = vec![
"-",
"Research assistant ",
"PhD candidate ",
"Post-doc researcher ", 
"Senior lecturer ", 
"Dean ",
];

let teacher = vec![
"Placement ",
"Classroom teacher ",
"Snr teacher ",
"Leading principal ",
"Deputy principal ",
"Pricipal ",
];


if profession == "O" {
println! ("You are an office administrator ");

if years_of_experience >= 1.0 && years_of_experience <= 2.0 {
   println! ("Your position is {} ", office_admin_vector [0]);
} else if years_of_experience >= 3.0 && years_of_experience <= 5.0 {
   println! ("Your position is {} ", office_admin_vector[1]);
} else if years_of_experience >= 5.0 && years_of_experience <= 8.0 { 
   println! ("Your position is {} ", office_admin_vector[2]);
} else if years_of_experience >= 8.0 && years_of_experience <= 10.0 { 
   println!("Your position is {} ", office_admin_vector[3]);
} else if years_of_experience >= 10.0 && years_of_experience <= 13.0 {
   println! ("Your position is {} ", office_admin_vector[4]);
} else if years_of_experience > 13.0 {
   println!("Your position is {}", office_admin_vector[5]);
}

} else if profession == "L" {
println! ("You are a lawyer ");
if years_of_experience >= 1.0 && years_of_experience <= 2.0 {
   println!("Your position is {}", lawyer[0]);
} else if years_of_experience >= 3.0 && years_of_experience <= 5.0 {
   println!("Your position is {}", lawyer[1]);
} else if years_of_experience >= 5.0 && years_of_experience <= 8.0 {
   println!("Your position is {}", lawyer[2]);
} else if years_of_experience >= 8.0 && years_of_experience <= 10.0 {
   println!("Your position is {}", lawyer[3]);
} else if years_of_experience >= 10.0 && years_of_experience < 13.0 {
   println!("Your position is {}", lawyer[4]);
} else if years_of_experience > 13.0 {
   println!("Your position is {}", lawyer[5]);
}

} else if profession == "A" {
println!("You are an Academic ");

if years_of_experience >= 1.0 && years_of_experience <= 2.0 {
    println!("Your position is {}", academic[0]);
} else if years_of_experience >= 3.0 && years_of_experience <= 5.0 {
    println!("Your position is {}", academic[1]);
} else if years_of_experience >= 5.0 && years_of_experience <= 8.0 {
    println!("Your position is {}", academic[2]);
} else if years_of_experience >= 8.0 && years_of_experience <= 10.0 {
    println!("Your position is {}",academic[3]);
} else if years_of_experience >= 10.0 && years_of_experience <= 13.0 {
    println!("Your position is {}", academic[4]);
}else if years_of_experience > 13.0 {
    println!("Your position is {}", academic[5]);
}

} else if profession == "T" {
println! ("You are a Teacher ");

if years_of_experience >= 1.0 && years_of_experience <= 2.0 { 
    println!("Your position is {}", teacher[0]);
} else if years_of_experience >= 3.0 && years_of_experience <= 5.0 {
    println!("Your position is {}", teacher[1]);
} else if years_of_experience >= 5.0 && years_of_experience <= 8.0 {
    println!("Your position is {}", teacher[2]);
} else if years_of_experience >= 8.0 && years_of_experience <= 10.0 {
    println!("Your position is {}", teacher[3]);
} else if years_of_experience >= 10.0 && years_of_experience <= 13.0 {
    println!("Your position is {}", teacher[4]);
}else if years_of_experience > 13.0 {
    println!("Your position is {}", teacher[5]);
}

} else {
    println!("null profession!");
}
}
