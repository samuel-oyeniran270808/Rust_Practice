use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the number guessing game!");

        let secret_number: u32 = rand::thread_rng().gen_range(1, 100);

    loop{
        
        println!("Please, input a number: ");

        let mut guess= String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed to read a line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        
        println!("You guessed {}", guess);

        
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oops. Too small!"),
            Ordering::Greater => println!("Haha, too big!"),
            Ordering::Equal => {
                println!("Yay! You win.");
                break;
            }
        }

    }
    
}