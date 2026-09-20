use std::io;

fn main() {
    println!("Employee Incentive Calculator ");

    // Ask for experience status
    println!("Is the employee experienced? (enter 'yes' or 'no'):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    if is_experienced {
        // Read age if the employee is experienced
        println!("Enter employee age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        let age: u32 = age_input.trim().parse().expect("Invalid integer entered");

        // Determine incentive based on age criteria
        if age >= 40 {
            println!("Incentive is N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Incentive is N1,400,000");
        } else if age < 28 {
            println!("Incentive is N1,300,000");
        } else {
            // Handles ages 28–29 which fall outside the given ranges
            println!("No specific incentive tier defined for experienced employees aged 28 to 29.");
        }
    } else {
        // Inexperienced employee
        println!("Incentive is N100,000");
    }
}