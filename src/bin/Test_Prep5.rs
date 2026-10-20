use std::io;

fn main() {
    println!("Welcome, user");
    println!("          ===================== MENU =====================");
    println!("      ITEM CODE           ITEM                       PRICE");
    println!("      P           Poundo Yam/Edinkaiko Soup         #3,200 \n 
      f           Fried Rice & Chicken              #3,000 \n 
      a           Amala & Ewedu Soup                #2,500 \n 
      e           Eba & Egusi Soup                  #2,000 \n 
      w           White Rice & Stew                 #2,500 \n ");

    let mut total: u32 = 0;

    loop {
        println!("Enter item code (or q to finish): ");

        let mut user_input =String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read input");

        let code = user_input.trim();

        if code == "q" {
            break;
        } 


        let price: u32 = match code {
            "p" => 3200,
            "f" => 3000,
            "a" => 2500,
            "e" => 2000,
            "w" => 2500,
            _=> {
                println!("Invalid item code");

                continue;
            }
        };

        println!("Enter quantity:");

        let mut quantity = String::new();

        io::stdin().read_line(&mut quantity).expect("Failed to read quanntity");

        let quantity: u32 = quantity.trim().parse().expect("Please enter a number");

        total += price * quantity;

         println!("Your total is: {}", total);
    }

   

}
