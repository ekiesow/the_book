#![allow(unused_variables)]
fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1 // expressions do not include ending semic
    }; // block that evaluates to 4

    let x = five();
    println!("The value of x from five() is: {}", x);

    let x = six();
    println!("The value of x from six() is: {}", x);

    let x = plus_one(x);
    println!("The value of x after plus_one() is: {}", x);
}

// Rust convention to use snake case for function (and variable) names
// In funtion signatures, you must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    println!("Another function printing x: {}", x);
    println!("Another function printing y: {}", y);
}

/*
 * Statements: instructions that perform some action and do not return a value
 * 
 * let y = 6; // does not return a value
 * let x = (let y = 6); // does not work as let keyword does not return a value
 * function definitions are also statements
 */

/*
 * Expressions: evaluate to a resulting value
 * do not end in a semicolon, semic will turn expression into statement
 *
 * 5 + 6 // expression that evaluates to 11
 * 6 in let y = 6 is an expression that evaluates to 6
 * calling a function is an expression
 * calling a macro is an expression
 * block used to create new scopes, {}, is an expression
 */

 /* functions with return values*/

// functions return the last expression implicitly
fn five() -> i32 {
    5
}

// use the return keyword to return early
fn six() -> i32 {
    if true {
        return 6
    }
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //; <-- this  would be a statement and produce a compiler error
}
