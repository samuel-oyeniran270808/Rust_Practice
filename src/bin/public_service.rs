use std::io;

fn main() {

    let level = vec!["APS 1-2", "APS 3-5", "APS 5-8", "ELI 8-10", "EL2 10-13", "SES"];

    let office_admin = vec!["Intern", "Administrator", "Senior administrator", "Office Manager", "Director", "CEO"];

    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Ppost-Doc Researcher", "Senior Lecturer", "Dean"];

    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!(" \n ============Positions=========== \n 
        1      office admin \n 
        2      academic \n 
        3      lawyer \n 
        4      teacher");

    println!("What kind of public servant are you?");
    
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read user's input");

    let user_input = user_input.trim();

    let profession = match user_input {
        "1" => "office admin",
        "2" => "academic",
        "3" => "lawyer",
        "4" => "teacher",
        _ => {
            println!("Invalid input");
            return;
        }
    };

    let mut experience = String::new();

    println!("Please, input your years of experience:");

    io::stdin().read_line(&mut experience);

    let exp: u32 = experience.trim().parse().expect("Failed to convert to number");
    
    let index = match exp {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        9..=10 => 3,
        11..=13 => 4,
        _ => 5,
    };

    println!("Experience: {} years", exp);

    println!("Level: {}", level[index]);

    println!("Job title: {}", profession[index]);

}