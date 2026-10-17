fn main() {
    // A vector of tuples: (name, years of programming experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Ask how many people are being interviewed
    let mut input = String::new();
    println!("How many candidates are you interviewing?");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let count: u32 = input.trim().parse().expect("Invalid input");

    // Read each candidate as the interview happens and push them in
    for n in 0..count {
        let mut name_input = String::new();
        println!("Enter name of candidate {}:", n + 1);
        std::io::stdin().read_line(&mut name_input).expect("Failed to read input");
        let name: String = name_input.trim().to_string();

        let mut years_input = String::new();
        println!("Enter years of programming experience for {}:", name);
        std::io::stdin().read_line(&mut years_input).expect("Failed to read input");
        let years: u32 = years_input.trim().parse().expect("Invalid input");

        candidates.push((name, years));
    }

    // Guard: nothing to compare if nobody was entered
    if candidates.len() == 0 {
        println!("No candidates were entered.");
        return;
    }

    // Walk the vector, remembering the POSITION of the best candidate so far
    let mut best_index: usize = 0;
    for i in 1..candidates.len() {
        if candidates[i].1 > candidates[best_index].1 {
            best_index = i;
        }
    }

    println!(
        "{} has the most experience: {} years",
        candidates[best_index].0, candidates[best_index].1
    );
}