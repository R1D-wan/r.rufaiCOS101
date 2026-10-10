
use std::io;

fn get_input(prompt: &str) -> f64 {
    let mut input = String::new();

    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Please enter a valid number")
}

// 1. Area of a trapezium
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

// 2. Area of a rhombus
fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// 3. Area of a parallelogram
fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// 4. Surface area of a cube
fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// 5. Volume of a cylinder
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("===== THE SHAPE CALCULATOR =====");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface Area of a Cube");
    println!("5. Volume of a Cylinder");

    let mut choice = String::new();

    println!("Enter your choice (1-5):");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");

    let choice: u32 = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid choice.");
            return;
        }
    };

    match choice {
        1 => {
            let height = get_input("Enter the height:");
            let base1 = get_input("Enter the first base:");
            let base2 = get_input("Enter the second base:");

            let answer = trapezium_area(height, base1, base2);
            println!("Area of trapezium = {:.2}", answer);
        }

        2 => {
            let diagonal1 = get_input("Enter the first diagonal:");
            let diagonal2 = get_input("Enter the second diagonal:");

            let answer = rhombus_area(diagonal1, diagonal2);
            println!("Area of rhombus = {:.2}", answer);
        }

        3 => {
            let base = get_input("Enter the base:");
            let altitude = get_input("Enter the altitude:");

            let answer = parallelogram_area(base, altitude);
            println!("Area of parallelogram = {:.2}", answer);
        }

        4 => {
            let side = get_input("Enter the side length:");

            let answer = cube_surface_area(side);
            println!("Surface area of cube = {:.2}", answer);
        }

        5 => {
            let radius = get_input("Enter the radius:");
            let height = get_input("Enter the height:");

            let answer = cylinder_volume(radius, height);
            println!("Volume of cylinder = {:.2}", answer);
        }

        _ => println!("Invalid choice. Please select a number from 1 to 5."),
    }
}
