// LESSON 1: WHAT IS A FUNCTION?
//
// A function is a named set of instructions. Think of it like a small machine:
// when we call the machine, it performs the instructions inside it.
// Keeping instructions in a function lets us give that job a name and reuse it.

// `main` is the starting function for this runnable program.
// Rust begins running a normal program by calling `main`.
fn main() {
    // This is a function call: the function's name followed by parentheses.
    // Calling `greet` tells Rust to run the instructions in its function body.
    greet();
    say_goodbye();

    // Rust runs this line after both function calls above have finished.
    println!("We are back inside main.");
}

// This is a function definition. `fn` tells Rust that we are defining a function.
// `greet` is the function's name. The empty parentheses mean it takes no inputs.
// The curly braces `{}` contain the instructions that run when `greet` is called.
// This definition appears after `main`, but Rust still knows about it when compiling.
fn greet() {
    // This instruction runs only because `main` called `greet`.
    // `println!` displays a line of text in the terminal. The `!` is part of
    // the macro's name; a macro is a Rust construct that helps generate code.
    println!("Hello from the greet function!");
}

fn say_goodbye() {
    println!("Goodbye from the goodbye function!");
}