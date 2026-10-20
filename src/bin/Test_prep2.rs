use std::io;
/* 
Question 2 (35 marks) – Student Loan Repayment Estimator

A microfinance bank has employed you to work on their student loan repayment estimator. They explain that a student borrows an amount, P, at an annual interest rate, R (%) for T years. The amount due after T years is:

\[ A = P \times (1 + R/100) \cdot T \]

You are tasked to write a Rust program that meets these requirements:

1. Accept P, R, and T via keyboard input.  
2. Computes both A and the local interest (A - P).  
3. Asks if the user wants to calculate for another borrower. Continue if "y", end if "n".  
4. Use appropriate type casting and loop.

*Suggested time:* 40 minutes
*/

fn main() {

    let mut p = String::new();
    let mut r = String::new();
    let mut t = String::new();


    println!("Input your principal");
    io::stdin().read_line(&mut p).expect("Failed to read principal");
    let p: f64 = p
        .trim()
        .parse()
        .expect("Not a valid input");
    println!("Input your rate");
    io::stdin().read_line(&mut r).expect("Failed to read rate");
    let r: f64 = r
        .trim()
        .parse()
        .expect("Not a valid input");
    println!("Input time");
    io::stdin().read_line(&mut t).expect("Failed to read time");
    let t: f64 = t
        .trim()
        .parse()
        .expect("Not a valid input");

    let a: f64 = p * (1.00 + r/100.00) * t; 

    let interest: f64 = a - p;

    println!("Your loan is {}", a);
    println!("Your interest is {}", interest);


}