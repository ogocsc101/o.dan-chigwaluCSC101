use std::io::Write;

fn main(){
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC102111","ECO101101","CSC103288","EEE110202","MEE102020"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec![300,100,200,200,100];

    let mut file = std::fs::File::create("Student_Management_Information_System.txt").expect("create failed");

    for x in 0..student_name.len(){
        file.write_all(format!("{} {} {} {}\n",student_name[x],matric_number[x],department[x],level[x]).as_bytes()).expect("Failed to write");
    } 
}