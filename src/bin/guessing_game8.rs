use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the number guessing game!");
    let secret_guess: u32 = rand::thread_rng().gen_range(1, 51);


    loop {
        println!("Please input your guess here: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_guess) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

    }
    

}