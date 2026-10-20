
use std::io;
use rand::Rng; // This is what bring in the external library that has the numbers because Rust doesn't have it. I don't know why though.
use std::cmp::Ordering;// This kinda works like the IF/ELSE conditions however, it follows an order of 'less than', 'greater than' or 'equal to'.

fn main() {


    println!("Welcome to the guessing game!");

    let secret_guess: u32 = rand::thread_rng().gen_range(1, 151);// This means that secret number will be generated with the "rand::thread_rng" and the "gen_range(1, 151)" gives the range at which the number will be selected at random. So the number will be between 1 and 150. 


    loop{ //This is to make everything in here to be in a loop unless specified to break aftera condition has been met like guessing the right number. 

        println!("Please input any guess(a number): ");

        let mut user_guess = String::new();// This creates a variable for whatever the user input but the "String::new()" keeps the variable empty and makes it a string. So the variable is empty untill the user inputs a string.
        
        io::stdin().read_line(&mut user_guess)// This is what allows the userr to input whatever they want.
        .expect("failed to read a line"); //This is what would print out if by chance, something unexpeccted happens i.e an error occurs.

        let user_guess: u32 = match user_guess.trim().parse() { //THe 'trim()' removes the whitespace before and after the value inputted while the parse changes it to another data type which in this case is unsigned integer. I think is also to deal with any unexpected error. I'm not too sure, i'll confirm.
            Ok(num) => num,
            Err(_) => continue, // This means that if the value isn't what is requested, it would ignore it and start the loop again
        }; // This is to convert the user's inputted guess from being a string to an unsigned integer. 

        println!("You guessed {}", user_guess);// This prints out the users guess.

        match user_guess.cmp(&secret_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

    }
    

}