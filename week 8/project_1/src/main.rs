
use std::io;

fn main() {
    let public_service = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "Not specified",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    println!("Welcome to the APS Level Checker!");
    println!("Enter a letter for your occupation:");
    println!("O - Office Admin");
    println!("L - Lawyer");
    println!("T - Teacher");
    println!("A - Academic");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let profession = input.trim().to_uppercase();

    println!("\nEnter your rank number:");
    for i in 0..6 {
        println!("{} - {}", i, public_service[i]);
    }

    let mut category = String::new();

    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read rank");

    let index: usize = match category.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Enter a number from 0 to 5.");
            return;
        }
    };

    if index >= public_service.len() {
        println!("Invalid rank. Choose a number from 0 to 5.");
        return;
    }

    let job_title = match profession.as_str() {
        "O" => office_admin[index],
        "L" => lawyer[index],
        "T" => teacher[index],
        "A" => academic[index],
        _ => {
            println!("Invalid occupation. Choose O, L, T, or A.");
            return;
        }
    };

    println!("\nOccupation: {}", profession);
    println!("Job title: {}", job_title);
    println!("Public Service Level: {}", public_service[index]);
}
