use std::io;

fn main() {
    //input for name
    let mut name_input = String::new();
    println!("\nEnter your name: ");
    io::stdin().read_line(&mut name_input).expect("Not a valid string");
   
    //input for age
    let mut age_input = String::new();
    println!("\nEnter your age");
    io::stdin().read_line(&mut age_input).expect("Not a valid input");
    let age:i32 = age_input.trim().parse().expect("Failed to read number");


    //input for experience
    let mut experience_input = String::new();
    println!("\nHow many years have you been working with our organization");
    io::stdin().read_line(&mut experience_input).expect("Not a valid input");
    let experience:i32 = experience_input.trim().parse().expect("Failed to read number");

    //conditions
    if experience >= 10 && age >= 40 {
        let elderly:i32 = 1_560_000;
        println!("You're eligible for an incentive of ₦{} per month", elderly);
    }
    
    else if experience >= 10 && age >= 30 && age < 40 {
    let middle_age:i32 = 1_480_000;
    println!("You're eligible for an incentive of ₦{} per month", middle_age);
    }
    
    else if experience >= 10 && age <= 28 {
    let youth:i32 = 1_300_000;
    println!("You're eligible for an incentive of ₦{} per month", youth);
    }
    
    else {
    let inexperienced:i32 = 100_000; 
    println!("You're eligible for an incentive of ₦{} per month", inexperienced);
    }
}
