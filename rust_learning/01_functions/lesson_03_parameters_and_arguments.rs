// LESSON 3: PARAMETERS AND ARGUMENTS
//
// A function can receive information when we call it, like a machine that is
// given an ingredient to work with. The named input in the function definition
// is called a parameter. The actual value we supply in a call is an argument.

fn main() {
    // Here, 7 is the argument: it is the value supplied to the function call.
    show_number(7);

    // A variable can also supply an argument. `score` holds the value 12.
    let score = 12;
    show_number(score);
}

// `number` is the parameter name. `i32` is its type: this parameter receives
// a signed whole number represented by Rust's 32-bit integer type.
// Rust asks us to write the type so it can check calls before the program runs.
fn show_number(number: i32) {
    // Inside this function, `number` refers to the value provided by the caller.
    println!("The number is {number}.");
}
