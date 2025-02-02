use std::io::Read;

fn main() {
    let mut name_of_commisioner = std::fs::File::open("Commisioner_name.txt").unwrap();
    let mut commisioner_contents = String::new();
    name_of_commisioner.read_to_string(&mut commisioner_contents).unwrap();
    
    let mut ministry = std::fs::File::open("Ministry.txt").unwrap();
    let mut ministry_contents = String::new();
    ministry.read_to_string(&mut ministry_contents).unwrap();

    let mut geopolitical_zone = std::fs::File::open("Geopolitical_zone.txt").unwrap();
    let mut geopolitical_contents = String::new();
    geopolitical_zone.read_to_string(&mut geopolitical_contents).unwrap();

    let commisioner_vec: Vec<&str> = commisioner_contents.split("\n").collect();
    let ministry_vec: Vec<&str> = ministry_contents.split("\n").collect();
    let geopolitical_vec: Vec<&str> = geopolitical_contents.split("\n").collect();

    for x in 0..commisioner_vec.len(){
        println!("{}     {}    {}", commisioner_vec[x].trim(), ministry_vec[x].trim(), geopolitical_vec[x].trim());
    }

}