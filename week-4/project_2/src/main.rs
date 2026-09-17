use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();


println!(" Are you experienced true or false");
io::stdin().read_line(&mut input1).expect("invalid input");
let experience:bool = input1.trim().parse().expect("invalid input");

println!("Enter age");
io::stdin().read_line(&mut input2).expect("invalid input");
let age:u32 = input2.trim().parse().expect("invalid input");

if experience == true && age >= 40 {
  let  _salary:f32 = 1560000.00;
  println!("your incentive is {}", _salary );
} 
else if experience == true && age >= 30 && age <= 39 {
  let  _salary:f32 = 1480000.00;
  println!("your incentive is {}", _salary );
}
else if experience == true && age <= 29 {
  let  _salary:f32 = 1300000.00;
  println!("your incentive is {}", _salary );
}
else if experience == false {
  let _salary:f32 = 100000.00;
  println!("your incentive is {}", _salary);
}
}