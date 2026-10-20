// LESSON 2: STATEMENTS AND EXPRESSIONS
//
// A statement performs an instruction, such as creating a variable.
// An expression works something out and has a value as its answer.
// Think of a statement as writing something into a notebook, and an expression
// as doing a calculation whose answer can be written there.

fn main() {
    // `let first_number = 4;` is a let statement.
    // It gives the name `first_number` to the value 4.
    // The semicolon marks the end of this instruction.
    let first_number = 4;

    // The braces make a block. This block is used as an expression because
    // its final line calculates a value that can be used by the outside code.
    let answer = {
        // This let binding is a statement inside the block.
        let extra = 1;

        // `first_number + extra` is the block's final expression.
        // It has the value 5. There is deliberately no semicolon here:
        // that lets the block provide this value to `answer`.
        first_number + extra
    };

    // `println!` displays the value that was calculated by the block.
    println!("The answer is {answer}.");
}
