use std::io;

fn main() {
    println!("Shape Calculator ");
    println!("Select a shape calculation:");
    println!("1 Area of Trapezium");
    println!("2 Area of Rhombus");
    println!("3 Area of Parallelogram");
    println!("4 Surface Area of Cube");
    println!("5 Volume of Cylinder");
    

    println!("Enter your choice (1-5):");
    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read input");
    let choice: u32 = choice_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    match choice { 
        1 => trapezium_area(),
        2 => rhombus_area(),
        3 => parallelogram_area(),
        4 => cube_surface_area(),
        5 => cylinder_volume(),
        _ => println!("Invalid choice! Please run the program again and select between 1 and 5."),
    }
}


fn trapezium_area() {
    println!("\nArea of Trapezium");
    let height = read_f32("Enter height: ");
    let base1 = read_f32("Enter base1: ");
    let base2 = read_f32("Enter base2: ");

    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}


fn rhombus_area() {
    println!("\nArea of Rhombus");
    let diagonal1 = read_f32("Enter diagonal 1: ");
    let diagonal2 = read_f32("Enter diagonal 2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus = {}", area);
}

fn parallelogram_area() {
    println!("\nArea of Parallelogram");
    let base = read_f32("Enter base: ");
    let altitude = read_f32("Enter altitude: ");

    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn cube_surface_area() {
    println!("\nSurface Area of Cube");
    let side = read_f32("Enter side length: ");

    let area = 6.0 * side * side;
    println!("Surface Area of Cube = {}", area);
}

fn cylinder_volume() {
    println!("\nVolume of Cylinder");
    let radius = read_f32("Enter radius: ");
    let height = read_f32("Enter height: ");

    let pi = std::f32::consts::PI;
    let volume = pi * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}

fn read_f32(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}