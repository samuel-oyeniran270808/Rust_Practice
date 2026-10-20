// LESSON 6: A FUNCTION CAN GIVE A VALUE BACK
//
// A function can do more than display a message. It can calculate an answer
// and give that answer back to the code that called it. Think of a machine that
// receives an input, does its job, and hands back a finished result.

fn main() {
    // The call to double_number(5) produces the value 10.
    // `let` stores that returned value under the name `answer`.
    let answer = double_number(5);

    // The returned value can now be used like any other i32 value.
    println!("The answer is {answer}.");
}

// `-> i32` tells Rust that this function gives back an i32 whole number.
fn double_number(number: i32) -> i32 {
    // This final expression calculates the result and gives it back to the caller.
    // It has no semicolon, so the function returns its value: 5 * 2 is 10.
    number * 2
}

// If we wrote `number * 2;` with a semicolon, that line would be a statement
// with no useful returned number. Rust would report a type mismatch because
// this function promises to return i32, but the body would provide unit `()`.
