use std::io;

fn main() {
    //initialize a vector to store candidate
    let mut candidates = Vec::new();

    //Simulate Candidate data input from user
    loop {
        println!("Enter candidate name (or 'quit' to finish):");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();
        if name.to_lowercase() == "quit" {break;}
        println!("Enter candidate's years of experience:");
        
        let mut years_of_experience = String::new();
        io::stdin().read_line(&mut years_of_experience).expect("Failed to read input");
        let years_of_experience:u32 = years_of_experience.trim().parse().expect("Invalid input");

        candidates.push((name.to_string(), years_of_experience));
    }

        //print candidate list
        println!("Candidate list:"); 
        for (i, (name, years_of_experience)) in candidates.iter().enumerate(){println!("{}. {} - {} years_of_experience",i +1, name, years_of_experience);
    }
    
    // Find the candidate with the highest years of experience
    let mut highest_experience =0;
    let mut highest_experience_candidate ="";
    for (name, years_of_experience) in &candidates { if *years_of_experience > highest_experience { highest_experience = *years_of_experience;
        highest_experience_candidate = name;
    }}
    //print the result 
    println!("The candidate with the highest years of programming experience is {} with {} years.", highest_experience_candidate, highest_experience);
}


<<<<<<< HEAD

=======
>>>>>>> da0e8ab90b3adfba594e294c13b3df374210bdd2
