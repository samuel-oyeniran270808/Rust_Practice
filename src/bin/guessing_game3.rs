use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    println!("Welcome to the number guessing game!");

    loop{

    let secret_number: u32 = rand::thread_rng().gen_range(1, 151);
    println!("please, input your guess: ");

    let mut guess = String::new();
    io::stdin().read_line( &mut guess)
    .expect("faled to read a line");


    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed {}!", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("Correct, you win!");
            break;
        }
      }

      
    }

}


