use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Dear User, elcome to the number guessing game!");
    println!("please create a user name: ");

    let mut user_name = String::new();

    io::stdin().read_line(&mut user_name)
    .expect("ffaled to read a line");

    println!("Your user name is {}", user_name);

    let secret_guess: u32 = rand::thread_rng().gen_range(1, 51);

    loop{

        println!("Please input your guess: ");
    
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("failed to read a line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_guess) {
            Ordering::Less => println!("Oops, too small ☹️, {user_name}"),
            Ordering::Greater => {
                println!("Too big, 😭  {user_name}. Try again");
                println!("Hint: Try something smaller: ");
            }
            Ordering::Equal => {
                println!("Yay, you guessed right!, {user_name} 🥳");
                println!("Do you want to play again? Y/N");

                let mut ans = String::new();

                io::stdin().read_line(&mut ans).unwrap(); /*
                The "unwrap()" is used to gve a successful value if the program works and it crashes if the program fails
                 */
                
                let ans: char = ans.trim().chars().next().unwrap(); /*
                Here, "chars()" turns string into an iterator over its character. It is a method on strings and string slices (&str) which i will learn more about later. It returns an iterator over the Unicode scalar values which are individual characters in the string.
                "next()" asks for the first character and if there is no first character, it returns a Option<char>: 
                some(Y) -> there was a charcter
                None -> there wasn't one.
                "unwrap()" simply asks for the character if one exists. If it doesn't, it crashes.
                 */

                if ans == 'Y' {
                    println!("Starting again!");
                    continue;
                }

                if ans == 'N' {
                    println!("Logging you out...");
                    break;
                }
            }
        }

        

    }
    
    

}