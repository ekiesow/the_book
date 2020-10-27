#![allow(unused)]
fn main() {
    // The if let syntax lets you combine if and let into a less verbose way to handle
    // values taht match one pattern while ignoring the rest.
    // Consider the code that matches on an Option<u8> value but only executes code
    // if the value is 3.
    {
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
    }

    // We want to do something with the Some(3) match but do nothing with any other
    // Some<u8> value or the None value. Instead we could write this in a shorter
    // way using the if let behaving the same as the match above.
    {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }

    // The if let syntax takes a pattern and an expression separated by an equal
    // sign. Chosing between match and if let depneds on what you're doing in the
    // particular situation and whether gaining conciseness is an appropriate
    // trade-off for losing exhaustive checking.

    // We can include an else with an if let that acts the same as the _ case in
    // the match expression. Recall the Coin enum where the Quarter variant also
    // held a UsState value. If we wanted to count all non-quarter coins we see
    // while also announcing the sate of the quarters, we could do that in a match
    // expression like this:
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or we could use an if let and else expression like this:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // If you have a situation in which your program has logic that is too verbose
    // to express using a match, remember that if let is in your Rust toolbox as
    // well.
}

/* Summary */
// Enums can be used to create custom types that can be one of a set of
// enumerated values.
// Standard library's Option<T> type helps you use the type system to
// prevent errors.
// You can use match or if let to extract data from enum values and use those
// values, depending on how many cases you need to handle.
