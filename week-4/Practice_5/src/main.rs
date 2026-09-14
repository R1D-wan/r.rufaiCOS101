// rust program to read the height of a person
// and the print if the person is tall , dwarf
// or average height person
use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("\n enter your height (in centimeters):");
    io::stdin().read_line(&mut input).expect ("not a valid string:");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("Your are an average ");
    }
    else if height >= 170.0 && height <= 195.0
    {
        println!("You are tall:");
    }
    else if height >= 150.0 && height <=100.0
    {
        println!("You are short:");
    }
    else {
        println!("Abnormal height:");
    }
}
