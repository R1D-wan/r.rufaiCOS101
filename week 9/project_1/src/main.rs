
use::std::io::Write;
fn main(){
 let title = "Lagar, Stout ,Non-Alchoholic \n";
    let row1 =  "33 Export, legend , Matina\n";
    let row2 =  "Desperados, Turbo king , Amstel maltina\n";
    let row3 =  "Gold Berg, Williams , Malta Gold\n";
    let row4 =  "Gulder,, Fayrouz\n";
    let row5 =  "Heineken,, \n";
    let row6 =  "Star,, \n";
let mut file = std::fs::File::create("Nigeran Breweries.csv").expect("create failed");
    file.write_all(title.as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");
    file.write_all(row6.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");

      println!("\n Data written to file");
    }