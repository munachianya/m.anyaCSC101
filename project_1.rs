//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {

    //declearing the var
    println!("Enter value a");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = a.trim().parse().expect("Not a valid Number");

    println!("\nEnter value b");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = b.trim().parse().expect("Not a valid Number");

    println!("\nEnter value c");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Not a valid Number");

    //Formula for discriminant
    let discriminant:f32 = (b.powf(2.0)) - 4.0 * a * c;
    let root1 = (-b + discriminant) / (2.0 * a);
    let root2 = (-b - discriminant) / (2.0 * a);  
    
    // conditional statement
    
    if a == 0.0 || b == 0.0 || c == 0.0 {
        println!("Error: Unable to determine roots");
        return;
    }
    else 
    {
    if discriminant > 0.0 {
            println!("Discriminant is positive. Roots are real and distinct, hence there are two roots");
            println!("Roots are {} and {}", root1, root2);
        }
        else if discriminant < 0.0 {
            println!("Discriminant is negative, hence there are no real roots");
        }
        else {
            println!("Discriminant is zero, hence there is exactly one root");
            println!("Root is {}",root1);
        }

    }
}