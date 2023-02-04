use std::io;

fn main() {
    println!("How many siblings do you have?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1 = input1.trim().parse().expect("Invalid input");

    println!("Enter the name and age of sibling");

    let mut names = vec![String::new(); input1];

    for x in 0..input1{
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        names[x] = name.trim().into();

        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let age:i32 = input2.trim().parse().expect("Invalid input");

        println!("Name of sibling is :{} and their age is :{}", name,age);

        if age > 18 {
            println!("Is this sibling married or single?");
            println!("Take 1 for single and 0 for married");

            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("Failed to read input");
            let answer:i32 = input4.trim().parse().expect("Invalid input");

            if answer == 1 {
                println!("Is this sibling a student or a worker?");
                println!("Take 1 for student and 0 for worker");

                let mut input5 = String::new();
                io::stdin().read_line(&mut input5).expect("Failed to read input");
                let profession:i32 = input5.trim().parse().expect("Invalid input");

                if profession == 1 {
                    println!("What university are you schooling?");
                    let mut uni = String::new();
                    io::stdin().read_line(&mut uni).expect("Failed to read input");
            
                    println!("What course are you studying?");
                    let mut course = String::new();
                    io::stdin().read_line(&mut course).expect("Failed to read input");
                }
                else if answer == 0 {
                println!("Do you have any offspring?");
                let mut offspring = String::new();
                io::stdin().read_line(&mut offspring).expect("Failed to read input");

                println!("What city do you and your family live in?");
                let mut city = String::new();
                io::stdin().read_line(&mut city).expect("Failed to read input");
                }
            }
        }
            else if age < 18 {
            println!("Have you written WAEC?");
            println!("Take 1 for yes and 0 for no");

            let mut input8 = String::new();
            io::stdin().read_line(&mut input8).expect("Failed to read input");
            let status:i32 = input8.trim().parse().expect("Invalid input");

            if status == 1 {
                println!("Input the name of secondary school attended: ");
                let mut school = String::new();
                io::stdin().read_line(&mut school).expect("Failed to read input");
            }
            else if status == 0 {
                println!("Input your current class level :");
                let mut class = String::new();
                io::stdin().read_line(&mut class).expect("Failed to read input");
            }
        }
    }
}

