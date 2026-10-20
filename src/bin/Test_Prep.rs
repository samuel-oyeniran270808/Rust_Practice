use std::io;

fn main() {
    /* You have been employed by an electricity distribution company (EKDC) to write a Rust program that calculates a customer’s monthly electricity bill based on their consumption in kilowatt-hours (kWh):

    | Units Consumed (kWh) | Rate per Unit (¥) |
    |---|---|
    | 0–100    | 20 |
    | 101-300    | 35 |
    | 301 and above    | 50 |

    *Requirements:*  
    ● Prompt the user to enter customer name and units consumed.  
    ● Use if...else to decide the rate and compute total.  
    ● If units > 500, add ¥6,000.00  
    ● Display the customer’s name, units, rate and total bill neatly */

    println!("Hello, user. Welcome to EKDC power tracker system!");

    let mut user_name = String::new();
    let mut units = String::new();

    println!("Please, input your name");
    io::stdin().read_line(&mut user_name).expect("Failed to read User name");

    println!("Input the your power usage amount in Kilowatts: ");
    io::stdin().read_line(&mut units).expect("Failed to read units used");

    let units: f64 = units.trim().parse().expect("Failed to read units used");

    let rate1: f64 = 50.00;
    let total_bill1: f64 = units * rate1 + 6_000.00;
    let rate2: f64 = 35.00;
    let total_bill2: f64 = units * rate2;
    let rate3: f64 = 20.00;
    let total_bill3: f64 = units * rate3;
    if units > 300.00 {
        println!("\nUSER NAME: {} \nUNITS OF POWER USED: {} \nRATE: {} \nTOTAL BILL: {}", user_name, units, rate1, total_bill1);
    }
    else if units > 100.00 && units < 301.00 {
        println!("\nUSER NAME: {} \nUNITS OF POWER USED: {} \nRATE: {} \nTOTAL BILL: {}", user_name, units, rate2, total_bill2);
    }
    else if units <= 100.00 {
        println!("USER NAME: {} \nUNITS OF POWER USED: {} \nRATE: {} \nTOTAL BILL: {}", user_name, units, rate3, total_bill3);
    }

}