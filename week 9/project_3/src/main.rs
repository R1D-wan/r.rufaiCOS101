use std::io::Write;

fn main() {

    // the first data set
    let data1 = vec![
        "COMMISSIONER",
        "Aigbodun Alamba Daudu",
        "Murtala Aleez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("Create failed");

    // the second data set
    
    let data2 = vec![
        "MINISTRY",
        "Internal affairs",
        "Justice",
        "Defence",
        "Power & steel",
        "Petroleum",
    ];

   // the third data set
    let data3 = vec![
        "ZONE",
        "South West",
        "Noth East",
        "South South",
        "South West",
        "South East",
    ];
    
    
         for i in 0..data1.len() 
         {
        let new_file = format!(
            "{}, {}, {}\n",
            data1[i], data2[i], data3[i]
            );
          file.write_all(new_file.as_bytes())
            .expect("Write failed");
    }
      println!("Data written to file");

}
