#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    /* Scalar Types */

    // integers: i8, u8, i/u16, i32 (default), u32, i/u64, i/u128
    let x = 5;
    println!("The value of x is: {}", x);
    // shadowing the variable x by using the let keyword
    // keeping x immutable after the transformations (x + 1)
    let x = x + 1;
    println!("The value of x is: {}", x);

    /*
     * constant
     * constants are immutable, declared using the const keyword
     * must always annotate a type
     * cannot be set to the result of a function call
     * or any value computed at runtime
     * Rust naming convention to use all uppercase with underscore between words
     */
    
     // can use underscore in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000; 

    // float: f32, f64 - double precision (default)
    let fl: f32 = 3.14159;
    let fl: f64 = 3.14159265359;

    // bool
    let t = true;
    let f: bool = false; // explicit type annotation

    // characters: Unicode Scalar Values
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    /* Compound Types */

    /*
     * tuple
     * cannot grow or shrink once declared and contain different types
     */
    let tup: (i32, f64, u8) = (500, 6.9, 12); // optional type annotations

    // destructuring a tuple through pattern matching to get individual values
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let five_hundred = tup.0; // access tuple element directly using index
    println!("The value of five_hundred is: {}", five_hundred);

    /*
     * array
     * fixed length like tuples but must contain values of the same type
     * data is allocated on stack
     */

    let a = [1, 2, 3, 4, 5];
    // array with type annotation
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //initialize an array with the same value for each element
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

    // accessing an array
    let first = a[0];
    let second = a[1];
    
    /*
     * arrays are useful when the size is unlikely to change
     * otherwise use a vector
     */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
