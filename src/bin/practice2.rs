
use std::io;

fn main() {
    println!(" \n
           MENU                        PRICES \n
    p = poundo yam and soup            3, 200 \n 
    f = Fried Rice & Chicken           3,000 \n 
    a = Amala & Ewedu Soup             2,500 \n 
    e = Eba & egusi soup               2,000 \n 
    w = White Rice & Stew              2,500
    \n");

    let mut code=String::new();
    let mut quantity = String::new();

    let p: u32 = 3_200;
    let f: u32 = 3_000;
    let a: u32 = 2_500;
    let e: u32 = 2_000;
    let w: u32 = 2_500;

    println!("Please, input your order: ");
    io::stdin().read_line(&mut code).expect("Failed to read code");

    println!("Please, input quantity");
    io::stdin().read_line(&mut quantity).expect("Failed to read quantity");
    let quantity: u32 = quantity.trim().parse().expect("Failed to read quantity"); 

    let code: char = code.trim().chars().next().expect("Failed to read code");

    if code == 'p' {
    let code = p;
    let total = code * quantity;
        if total > 10_000 {
            let discount: u32 = total * 5 / 100;
            let discount_total: u32 = total - discount;
            println!("{}", discount_total);
        }
        else {
            println!("{}", total);
        }
    
    }

    if code == 'f' {
    let code = f;
    let total2: u32 = code * quantity;
        if total2 > 10_000 {
                let discount2: u32 = total2 * 5 / 100;
                let discount_total2: u32 = total2 - discount2;
                println!("{}", discount_total2);
            }
            else {
                println!("{}", total2);
            }
    
    }

    if code == 'a' {
    let code = a;
    let total3: u32 = code * quantity;
        if total3 > 10_000 {
            let discount3: u32 = total3 * 5 / 100;
            let discount_total3: u32 = total3 - discount3;
            println!("{}", discount_total3);
        }
        else {
            println!("{}", total3);
        }
    }

    if code == 'e' {
    let code = e;
    let total4: u32 = code * quantity;
        if total4 > 10_000 {
            let discount4: u32 = total4 * 5 / 100;
            let discount_total4: u32 = total4 - discount4;
            println!("{}", discount_total4);
        }
        else {
            println!("{}", total4);
        }
    }

    if code == 'w' {
    let code = w;
    let total5: u32 = code * quantity;
        if total5 > 10_000 {
            let discount5: u32 = total5 * 5 / 100;
            let discount_total5: u32 = total5 - discount5;
            println!("{}", discount_total5);
        }
        else {
            println!("{}", total5);
        }
    }

}