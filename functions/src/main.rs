fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1 // expressions do not include ending semic
    }; // block that evaluates to 4 :0

    let x = five();
    println!("The value of x from five() is: {}", x);
    let x = plus_one(x);
    println!("The value of x after plus_one() is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("Another function printing x: {}", x);
    println!("Another function printing y: {}", y);
}

// functions return the last expression implicitly
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //; this would be a statement and produce a compiler error
}

/*
 * Statements: instructions that perform some action and do not return a value
 * let y = 6; // does not return a value, x = y = 6 does not work
 *
 */

/*
 * Expressions: evaluate to a resulting value
 * do not end in a semicolon, semic will turn expression into statement
 *
 * function definitions
 * 5 + 6 // expression that evaluates to 11
 * 6 in let y = 6 is an expression that evaluates to 6
 * calling a function is an expression
 * calling a macro is an expression
 * block used to create new scopes, {}, is an expression
 *
 */
