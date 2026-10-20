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


    println!("please, input the code of what you want to purchase: ");
    let p: u32 = 3_200;
    let f: u32 = 3_000;
    let a: u32 = 2_500;
    let e: u32 = 2_000;
    let w: u32 = 2_500;
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user's code input");
    let user_input: char = user_input.trim().chars().next().expect("Failed to convert user input");

    println!("Please, input the amount you want: ");
    let mut quantity = String::new();
    io::stdin()
        .read_line(& mut quantity)
        .expect("Failed to read quantity");
    let quantity: u32 = quantity.trim().parse().expect("Failed to convert quantity");

    if user_input == 'p' {
        let user_input = p;
        let total: u32 = user_input * quantity;
        if total > 10_000 {
            let discount: u32 = total * 10 / 100;
            let new_total: u32 = total - discount;
            println!("Your purchase worth: {}", new_total); 
        }
        else {

        println!("Your total is: {}", total);
        }
    } 

    if user_input == 'f' {
        let user_input = f;
        let total2: u32 = user_input * quantity;
        if total2 > 10_000 {
            let discount2: u32 = total2 * 10 / 100;
            let new_total2: u32 = total2 - discount2;
            println!("Your purchase worth: {}", new_total2); 
        }
        else {
            
        println!("Your total is: {}", total2);
        }
    } 

    if user_input == 'a' {
        let user_input = p;
        let total3: u32 = user_input * quantity;
        if total3 > 10_000 {
            let discount3: u32 = total3 * 10 / 100;
            let new_total3: u32 = total3 - discount3;
            println!("Your purchase worth: {}", new_total3); 
        }
        else {
            
        println!("Your total is: {}", total3);
        }
    } 

    if user_input == 'e' {
        let user_input = e;
        let total4: u32 = user_input * quantity;
        if total4 > 10_000 {
            let discount4: u32 = total4 * 10 / 100;
            let new_total4: u32 = total4 - discount4;
            println!("Your purchase worth: {}", new_total4); 
        }
        else {
            
        println!("Your total is: {}", total4);
        }
    } 

    if user_input == 'w' {
        let user_input = w;
        let total5: u32 = user_input * quantity;
        if total5 > 10_000 {
            let discount5: u32 = total5 * 10 / 100;
            let new_total5: u32 = total5 - discount5;
            println!("Your purchase worth: {}", new_total5); 
        }
        else {
            
        println!("Your total is: {}", total5);
        }
    } 
}