// rust program to count numbers

use std::io;

fn main() {
   println!("Enter lower boundry:");
   let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect ("Failed to read input:");
    let lower_boundry:i32 = input1.trim().parse().expect("Failed to input");

   println!("Enter upper boundry");
   let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect ("Failed to read input:");
    let upper_boundry:i32 = input2.trim().parse().expect("Failed to input:");

for x in lower_boundry..upper_boundry{
println!("count level is {} ",x );
}
}
