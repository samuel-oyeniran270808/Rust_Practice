fn main() {
    let x = 57i32;
    println!("{}", x);
    let remainder = 43 % 3;
    println!("{}", remainder);

    let a = 5;
    let b = {
    let a = 3;
    a + 1
    };
    println!("The value of b is: {}", b);
    println!("{}", a);

    for number in (1..4).rev() {
        println!("{}", number);
    }
}


