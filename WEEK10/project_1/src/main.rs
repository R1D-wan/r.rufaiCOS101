//declare a structure
struct laptop {
    brand : String,
    price: u32,
}
impl laptop { 

    fn totalcost (&self ,quantity:u64) -> u64 { self.price * quantity
        
}
 }
fn main() {
    let laptops = vec![
    //initialize a structure
    let device1 = laptop {
       brand: String::from("HP"),
        price: 650000,
    };
    let device2 = Electronics {
       brand: String::from("IBM"),
        price: 755000,
    };
    
    let device3 = laptop {
       brand: String::from("Toshiba"),
        price: 550000,
    };

    let device4 = laptop {
       brand: String::from("Dell"),
        price: 850000,
    };
    ];
    let quantity = 3;
    let mut total_cost = 0;
    println!("here is your laptop purchase summary");

    for laptop in &laptops {
     let cost = laptop.purchase_cost(quantity);
     total.cost +=cost;

     println!("Brand{} | Quantity {} | Total: {}", laptop.brand, quantity,cost );
 }
 println!("Grand Total {}, total_cost")", );