
use std::io;

fn main() {
    // Menu
    println!("Hello! What can we get for you today?\n");

    println!("--- RESTAURANT MENU ---");
    println!("P => Poundo Yam / Edinkaiko Soup-N3,200");
    println!("F => Fried Rice & Chicken- N3,000");
    println!("A => Amala & Ewedu Soup- N2,500");
    println!("E => Eba & Egusi Soup- N2,000");
    println!("W => White Rice & Stew- N2,500");
    
    println!("Please enter the code for your meal (P, F, A, E, W):");
    let mut code_input = String::new();
    io::stdin().read_line(&mut code_input).expect("Failed to read line");

    let choice = code_input.trim().to_uppercase();

    // price
    let unit_price: f64 = match choice.as_str() 
    {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
     _ =>  {   
            println!("Invalid code entered. Please restart and try again.");
            return;
        }
    };

    //  Quantity
    println!("How many portions would you like?");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");

    let quantity: f64 = match quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("Please enter a valid number for quantity.");
            return;
        }
    };

    // Calculate Total and Apply Discount
    let mut total_charge = unit_price * quantity;

    if total_charge > 10000.0 {
        let discount = total_charge * 0.05;
        total_charge -= discount;
        println!("\nCongratulations! You received a 5% discount for spending over N10,000.");
    }

    // Final Total
    println!("Your total charge is: N{}", total_charge);
}