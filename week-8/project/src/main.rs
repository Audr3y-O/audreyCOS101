use std::io;

fn div() {
    let o = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let a = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let l = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let t = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let p = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    // remember to add the code for years of experience

    let mut division = String::new();
    io::stdin().read_line(&mut division).expect("Failed to read input");
    let division = division.trim().to_lowercase();

    let title = match division.as_str() {
        "o" => &o,
        "a" => &a,
        "l" => &l,
        "t" => &t,
        _ => {
            println!("Please enter an input from the given options");
            return;
        }
    };

    println!("Enter your title from the following: {:?}", title);
    let mut spec = String::new();
    io::stdin().read_line(&mut spec).expect("Invalid input entered");
    let spec = spec.trim().to_lowercase();

    let mut found = false;
    for index in 0..title.len() {
        if title[index].to_lowercase() == spec {
            println!("Your APS level is {}", p[index]);
            found = true;
            break;
        }
    }

    if !found {
        println!("Title not found");
    }
}

fn main() {
    println!("Enter your division [o=Office Admin, a=Academic, l=Lawyer, t=Teacher]");
    div();
}