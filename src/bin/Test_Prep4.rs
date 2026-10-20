use std::io;


fn main() {
 println!(" \nCODE           ITEM        PRICES
 l              Laptop      550_000
 m              Monitor     120_000
 k              Keyboard    15_000
 h              Headset     25_000
 \n");

 let l: u32 = 550_000;
 let m: u32 = 120_000;
 let k: u32 = 15_000;
 let h: u32 = 25_000;
let mut code = String::new();


loop {

     println!("Please input the code of what you want to purchase: ");
    io::stdin().read_line(&mut code).expect("Failed to read code");
    let code: char = code.trim().chars().next().expect("Failed to read character");

    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failedd to read quantity");
    let quantity: u32 = quantity.trim().parse().expect("Failed to read quantity");

    if code == 'l' {
        let code = l;
        let total: u32 = code * quantity;
        println!("Your total is: {}", total);
        if total > 500_000 {
            let discount: u32 = total - 5 /100;
            let dscount_total: u32 = total - discount;
        }
    }
    if code == 'm' {
        let code = m;
        let total2: u32 = code * quantity;
        println!("Your total is: {}", total2);
        if total2 > 500_000 {
            let discount2: u32 = total2 - 5 /100;
            let dscount_total2: u32 = total2 - discount2;
        }
    }
    if code == 'k' {
        let code = k;
        let total3: u32 = code * quantity;
        println!("Your total is: {}", total3);
        if total3 > 500_000 {
            let discount3: u32 = total3 - 5 /100;
            let dscount_total3: u32 = total3 - discount3;
        }
    }
    if code == 'h' {
        let code = h;
        let total4: u32 = code * quantity;
        println!("Your total is: {}", total4);
        if total4 > 500_000 {
            let discount4: u32 = total4 - 5 /100;
            let dscount_total4: u32 = total4 - discount4;
        }
    }

}

}