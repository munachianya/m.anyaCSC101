use std::io;
    fn trapiezum (h: f32, b1: f32, b2: f32) {
    let area1:f32 = (h/2.0) * (b1 + b2);
    println!("Area of the Trapezium = {}", area1);
    }
    fn rhombus (d1: f32, d2: f32){
    let area2:f32 = 0.5 * d1 * d2;
    println!("Area of the Rhombus = {}", area2);
    }
    fn parallelogram (b: f32, a: f32){
    let area3:f32 = b * a;
    println!("Area of the Parallelogram = {}", area3);
    }
    fn cube (ls: f32){
    let area4:f32 = 6.0 * (ls.powf(2.0));
    println!("Area of Cube = {}", area4);
    }
    fn cylinder (r: f32, h1: f32){
    let pi = std:: f32::consts::PI;
    let area5:f32 = pi * (r.powf(2.0)) * h1;
    println!("Volume of Cylinder = {}", area5);
    }

fn main() {
    //user picking which formula to use
    let mut input0 = String::new();
    println!("Welcome to User! \nPlease select from the options which pre-made calculator you want.");
    println!("(1) Trapezium");
    println!("\n(2) Rhombus");
    println!("\n(3) Parallelogram");
    println!("\n(4) Cube");
    println!("\n(5) Cylinder");
    io::stdin().read_line(&mut input0).expect("Not a valid string");
    let function:i32 = input0.trim().parse().expect("Not a valid number");
    
    if function == 1 {
    println!("Calculating the area of a Trapezium");
    let mut input1 = String::new(); 
    println!("Input the value of the height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:f32 = input1.trim().parse().expect("Invalid input");
    
    let mut input2 = String::new(); 
    println!("Input the value of the 1st base:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b1:f32 = input2.trim().parse().expect("Invalid input");
    
    let mut input3 = String::new();
    println!("Input the value of the 2nd base:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b2:f32 = input3.trim().parse().expect("Invalid input");

    trapiezum(h, b1, b2)
    }
    else if function == 2 {
    println!("Calculating the area of a Rhombus"); 
    let mut input4 = String::new(); 
    println!("Input the value of the 1st diagonal:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let d1:f32 = input4.trim().parse().expect("Invalid input");
    
    let mut input5 = String::new();
    println!("Input the value of the 2nd diagonal:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let d2:f32 = input5.trim().parse().expect("Invalid input");

    rhombus(d1, d2)
    }
    else if function == 3 {
    println!("Calculating the area of a Parallelogram"); 
    let mut input6 = String::new(); 
    println!("Input the value of the base:");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let b:f32 = input6.trim().parse().expect("Invalid input");
    
    let mut input7 = String::new();
    println!("Input the value of the altitude:");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let a:f32 = input7.trim().parse().expect("Invalid input");

    parallelogram(a, b)
    }
    else if function == 4 {
    println!("Calculating the area of a Cube"); 
    let mut input8 = String::new(); 
    println!("Input the value of the length of the side:");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let ls:f32 = input8.trim().parse().expect("Invalid input");

    cube(ls);
    }
    else if function == 5 {
    println!("Calculating the area of a Cylinder"); 
    let mut input9 = String::new(); 
    println!("Input the value of the radius:");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let r:f32 = input9.trim().parse().expect("Invalid input");
    
    let mut input10 = String::new();
    println!("Input the value of the height:");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let h1:f32 = input10.trim().parse().expect("Invalid input");

    cylinder(r, h1)  
    }
    else {
        println!("Please pick a number from the available options");
    }

}