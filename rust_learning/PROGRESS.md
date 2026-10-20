# Rust Learning Progress

## Lessons

- Lesson 1, What Is a Function: introduced; first exercise completed.
- Lesson 2, Statements and Expressions: introduced; identification exercise completed.
- Lesson 3, Parameters and Arguments: completed; identified two parameters and their arguments and calculated the output.
- Lesson 4, Multiple Parameters: completed; correctly traced argument positions.
- Lesson 5, Parameter Types: completed; identified the values and why swapped argument types fail. Clarified that `true` is a Boolean value, while `"true"` is text.
- Lesson 6, Return Values: introduced; exercise pending.

## Concepts Introduced

- A function is a named set of instructions.
- `main` is where a normal binary program begins.
- `fn` defines a function; calling a function runs its body.
- A function may be defined after the place where it is called.
- Instructions run in execution order, not simply in file order.
- A `let` binding is a statement that gives a value a name.
- An expression computes a value; a block can use its final expression as its result.
- A semicolon ends an expression as an instruction, so a block with a final semicolon does not provide that expression's value.
- A parameter is a named input declared by a function; an argument is the value supplied at a call.
- Rust function parameters have written types so Rust can check that each call supplies a compatible value.
- When calling a function with multiple parameters, arguments match parameters from left to right by position.
- Each argument must have a type compatible with the parameter in that position; Rust reports mismatches at compile time.
- A Boolean literal such as `true` is not a string; quotation marks would make `"true"` text instead.
- A function can return a value with a declared return type after `->`; its final expression supplies that value when it has no semicolon.

## Demonstrated Understanding

- Correctly predicted that `greet` runs first, `say_goodbye` runs next, and `main` then continues to its next print statement.
- Correctly identified the `say_goodbye` function definition and the separate call that runs it.
- Correctly identified the whole `let total = 3 + 4;` line as a statement and `3 + 4` as its expression.
- Correctly identified the whole `let doubled = 5 * 2;` line as a statement and `5 * 2` as its expression.
- Correctly identified `first` and `second` as parameters, `3` and `4` as arguments, and predicted their sum is `7`.
- Correctly traced `show_pair(8, 2)`: `first` receives `8` and `second` receives `2`.
- Correctly matched `16.5` and `true` to the `f64` and `bool` parameters and recognized that reversing them causes a compile-time type mismatch; clarified Boolean versus text notation.

## Exercises

- Lesson 1 exercise: completed; predicted and reported the three output lines in the correct order.
- Lesson 2 exercise: completed; identified a let statement and the calculation expression inside it.
- Lesson 3 exercise: completed; identified parameters and arguments in a two-input function call and predicted the printed sum.
- Lesson 4 exercise: completed; correctly predicted parameter values from the supplied argument order.
- Lesson 5 exercise: completed; identified the parameter values and the incompatible swapped call.
- Lesson 6 exercise: pending; predict a returned value and explain what a semicolon would change.

## Projects

- None started.

## Revisit

- Continue with return values and practice how the final expression and semicolon affect what a function returns.
