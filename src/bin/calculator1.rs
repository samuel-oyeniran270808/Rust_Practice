use std::io;
fn main() {
    println!("=============CALCULATOR============");

    loop {

        let mut num1 = String::new();
        let mut num2 = String::new(); 
        let mut user = String::new();

        println!("Please, input first number: ");
        io::stdin().read_line(&mut num1).unwrap();
        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter an operator( +, -, *, /): ");
        io::stdin().read_line(&mut user)
        .expect("failed to read a line");
        let user: char = user.trim().chars().next().unwrap();

        println!("Please, input second number: ");
        io::stdin().read_line(&mut num2).unwrap();
        let num2: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        


        if user == '+' {
            let addition: i32 = num1 + num2;
            println!("{addition}");
            println!("Do you wish to continue? (Y/N): ");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).unwrap();
            let ans: char = ans.trim().chars().next().unwrap(); 
            if ans == 'N' {
                break;
            }
        }
        
        else if user == '-' {
            let subtraction: i32 = num1 - num2;
            println!("{subtraction}");
            println!("Do you wish to continue? (Y/N): ");
            let mut ans2 = String::new();
            io::stdin().read_line(&mut ans2).unwrap();
            let ans2: char = ans2.trim().chars().next().unwrap(); 
            if ans2 =='N' {
                break;
            }
        }

        else if user == '*' {
            let product: i32 = num1 * num2;
            println!("{product}");
            println!("Do you wish to continue? (Y/N): ");
            let mut ans3 = String::new();
            io::stdin().read_line(&mut ans3).unwrap();
            let ans3: char = ans3.trim().chars().next().unwrap(); 
            if ans3 =='N' {
                break;
            }
        }

        else if user == '/' {
            if num2 == 0 {
                println!("Invalid Operation");
            } else {
                let division: i32 = num1 / num2;
                println!("{}", division);
            }
            println!("Do you wish to continue? (Y/N): ");
            let mut ans4 = String::new();
            io::stdin().read_line(&mut ans4).unwrap();
            let ans4: char = ans4.trim().chars().next().unwrap(); 
            if ans4 =='N' {
                break;
            }
        }
        else {
            break;
        }

    }
    
}