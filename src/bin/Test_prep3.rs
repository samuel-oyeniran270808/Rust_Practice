use std::io;

fn main() {
    /*
    Campus Café Order System

    The PAU Café needs a talented problem solver to help them with an Ordering System complete with discount calculators and they have approached you to assist them.

    The Café sells the following:

    | Code    | Item    | Price (¥) |
    |---|---|---|
    | T    | Tea    | 800 |
    | C    | Coffee    | 1,200 |
    | S    | Sandwich    | 2,000 |
    | J    | Juice    | 1,500 |

    *Program tasks:*

    1. Display the café menu.  
    2. Ask the customer for item code and quantity.  
    3. Compute total cost.  
    4. If total ≥ ¥5,000, apply a 5% discount.  
    5. Display the final amount.  
    6. Allow multiple entries until the user types “exit”.

    *Hints:*  
    - Use match or if...else to determine price.  
    - Use variables, loop, and mutability.  
    - Allow multiple entries until the user chooses to quit.

    *Suggested time:* 45 minutes
*/

    println!("CODE          ITEM        PRICE \nt              Tea           800 \nc              Coffee        1,200 \ns              Sandwish      2,000 \nj              Juice         1,500");



    let tea: i32 = 800;
    let coffee: i32 = 1_200;
    let sandwish: i32 = 2_000;
    let juice: i32 = 1_500;


    let mut item_code= String::new();
    println!("Please input item code: ");
    io::stdin().read_line(&mut item_code).expect("Failed to read item code");
    let item_code: char = item_code.trim().chars().next().expect("failed to read item_code");

    let mut quantity = String::new();
    println!("Please, input quantity: ");
    io::stdin().read_line(&mut quantity).expect("Faledd to read quantity");
    let quantity: i32 = quantity.trim().parse().expect("Failed to read quantity");

    
    let c: i32 = coffee;
    let s: i32 = sandwish;
    let j: i32 = juice;

    if item_code == 't' {
        let t: i32 = tea;
        let t_total: i32 = t * quantity; 
        println!("Your total is: {}", t_total);
    }
    if item_code == 'c' {
        let c_total: i32 = c * quantity; 
        println!("Your total is: {}", c_total);
    }
    if item_code == 's' {
        let s_total: i32 = s * quantity; 
        println!("Your total is: {}", s_total);
    }
    if item_code == 'j' {
        let j_total: i32 = j * quantity; 
        println!("Your total is: {}", j_total);
    }

}