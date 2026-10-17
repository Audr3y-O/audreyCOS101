use std::f64::consts::PI;
use std::io;

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let value: f64 = input.trim().parse().expect("Invalid input");
    value
}

// 1. Trapezium, area = height / 2 * (base1 + base2)
fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus, area = 1/2 * diagonal1 * diagonal2
fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter diagonal 1:");
    let diagonal2 = read_number("Enter diagonal 2:");
    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram, area = base * altitude
fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    base * altitude
}

// 4. Cube, surface area = 6 * side * side
fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side length:");
    6.0 * side * side
}

// 5. Cylinder, volume = pi * radius * radius * height
fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    PI * radius * radius * height
}

fn main() {
    println!("===== Shape Calculator =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");

    let choice = read_number("Enter your choice (1-5):");

    if choice == 1.0 {
        println!("Area of trapezium = {:.2}", trapezium_area());
    } else if choice == 2.0 {
        println!("Area of rhombus = {:.2}", rhombus_area());
    } else if choice == 3.0 {
        println!("Area of parallelogram = {:.2}", parallelogram_area());
    } else if choice == 4.0 {
        println!("Surface area of cube = {:.2}", cube_surface_area());
    } else if choice == 5.0 {
        println!("Volume of cylinder = {:.2}", cylinder_volume());
    } else {
        println!("Invalid choice. Please run the program again and pick 1 to 5.");
    }
}