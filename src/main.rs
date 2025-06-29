use std::{io};

fn main() {
    loop{
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();
    let result: i32;

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid input");
    let x: i32 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            continue;
        }
    };
    

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid_input");
    let y: i32 = match y.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            continue;
        }
    };

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut op).expect("Invalid_input");
    let op: i32 = match op.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            continue;
        }
    };

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Invalid selection");
            continue;
        }
    }

    println!("The result is: {}", result);
}
}
