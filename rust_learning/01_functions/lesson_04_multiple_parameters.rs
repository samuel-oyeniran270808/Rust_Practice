// LESSON 4: MULTIPLE PARAMETERS AND ARGUMENT ORDER
//
// A function can receive more than one input. The first argument goes to the
// first parameter, the second argument goes to the second parameter, and so on.
// Think of two labeled boxes: Rust fills them from left to right at the call.

fn main() {
    // This call supplies 3 as the first argument and 4 as the second argument.
    show_pair(3, 4);

    // Swapping the arguments changes which value each parameter receives.
    show_pair(4, 3);
}

// `first` and `second` are parameters. Each must receive an i32 whole number.
fn show_pair(first: i32, second: i32) {
    // The names let us see which argument arrived in each position.
    println!("first is {first}; second is {second}.");
}
