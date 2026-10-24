use std::io::Write;

fn main() {
    let title = vec![
        "STUDENTS-NAME, MATRIC-NUMBER, DEPARTMENT, LEVEL\n"
    ];

    let rows = vec![
        "Oluchi-Mondi, ACC10211111, Accounting, 300\n",
        "Adams-Aliyu, ECO10110101, Economics, 100\n",
        "Shania-Bolade, CSC10328828, Computer, 200\n",
        "Adekunle-Gold, EEE11020202, Electrical, 200\n",
        "Blanca-Edemol, MEE10202001, Mechanical, 200\n",
    ];

    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("Create failed");

    // the title
    for item in title.iter() {
        file.write_all(item.as_bytes()).expect("Write failed");
    }

    //  the student records
    for item in rows.iter() {
        file.write_all(item.as_bytes()).expect("Write failed");
    }

    println!("Data written to file");
}
