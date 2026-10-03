use std::io;

fn main() {
    // Restaurant menu
    println!("WELCOME TO OUR RESTAURANT , HERE'S OUR MENU");
    println!("P = Poundo Yam / Edikaiko Soup  N3,200");
    println!("F = Fried Rice & Chicken        N3,000");
    println!("A = Amala & Ewedu Soup          N2,500");
    println!("E = Eba & Egusi Soup            N2,000");
    println!("W = White Rice & Stew            N2,500");
    println!("---------------------------------------");

    // food types
    println!("Enter food type (P, F, A, E, W):");

    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");

    let food = food.trim().to_uppercase();

    
    println!("Enter quantity:");

    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

    let quantity: f64 = quantity_input.trim().parse().expect("Please enter a valid number");

    let price: f64 = match food.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food type.");
            return;
        }
    };

    
    let total = price * quantity;

    println!("Total before discount: N{}", total);

    // Apply 5% discount if total is greater than N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount: N{}", discount);
        println!("Final total: N{}", final_total);
    } else {
        println!("No discount.");
        println!("Final total: N{}", total);
    }
}