// program to input values to find to roots of a quadratic equation
// and classify the roots gotten 
 use  std::io ;
fn main() {

    let mut input1= String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter a value for a :", );
    io::stdin().read_line(&mut input1).expect("input not valid");
    let a:f32 = input1.trim().parse().expect("input not valid");

    println!("enter a value for b :", );
    io::stdin().read_line(&mut input2).expect("input not valid");
    let b:f32 = input2.trim().parse().expect("input not valid");

    println!("enter a value for c :", );
    io::stdin().read_line(&mut input3).expect("input not valid");
    let c:f32 = input3.trim().parse().expect("input not valid");
 
    let d:f32 = b*b - 4.0 * a * c ;
     if d > 0.0 {
        println!("two distinct roots");
     }
     else if d == 0.0 {
        println!("exactly one real root");
     }
     else {
        println!("no real roots");
     }

}
