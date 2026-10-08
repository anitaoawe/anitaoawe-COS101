
use std::io;

fn add(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum of A and B = {sum}");
}

fn read_i32(prompt: &str) -> i32 {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .expect("Please enter a valid integer")
}

fn main() {
    let a = read_i32("Enter input for parameter A:");
    let b = read_i32("Enter input for parameter B:");

    // Call add function with parsed arguments
    add(a, b);
}