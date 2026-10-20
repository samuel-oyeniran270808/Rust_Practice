// LESSON 5: PARAMETERS CAN HAVE DIFFERENT TYPES
//
// A parameter's type tells Rust what kind of value belongs in that position.
// One function can receive different kinds of information, as long as each
// argument matches the type written beside its parameter.

fn main() {
    // 21.5 is a decimal number, and true is a yes/no value.
    // They match the first and second parameter types in describe_weather.
    describe_weather(21.5, true);

    // The values can change, but each position must still have the right type.
    describe_weather(18.0, false);
}

// `temperature` accepts an f64, a floating-point number for decimal values.
// `is_raining` accepts a bool, which can be true or false.
fn describe_weather(temperature: f64, is_raining: bool) {
    // Each placeholder displays the value received by its matching parameter.
    println!("Temperature: {temperature} degrees; raining: {is_raining}.");
}

// A call such as `describe_weather(true, 21.5)` would be a compile-time type
// error: the first argument is a bool, but the first parameter requires f64.
// Rust checks this before running the program, so mismatched values do not run.
