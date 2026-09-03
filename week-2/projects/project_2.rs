fn main() {
	let tosum:f64 = 2.0* 450000.00; // im using the first two letters of the items
	let masum:f64 = 1.0 * 1500000.00;
	let hpsum:f64 = 3.0 * 750000.00;
	let delsum:f64 = 3.0 * 2850000.00;
	let acesum:f64 = 1.0 * 250000.00;
	let totalsum = tosum + masum + hpsum + delsum + acesum;
	
    let totalqty:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0 ;

    let avg = totalsum/totalqty ;
    println!("The total average is {} and the total sum is {}", avg,totalsum );	

}