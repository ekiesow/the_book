#![allow(unused_variables)]
fn main() {
    /* Scalar Types */

    // integers: i8, u8, i/u16, i32 (default), u32, i/u64, i/u128
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);

    // float: f32, f64 - double precision (default)
    let fl: f32 = 3.14159;
    let fl: f64 = 3.14159265359;

    // bool
    let t = true;
    let f: bool = false;

    // characters: Unicode Scalar Values
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    /* Compound Types */

    // tuple
    let tup: (i32, f64, u8) = (500, 6.9, 12);
    let (x, y, z) = tup; // destructuring a tuple through pattern matcing
    println!("The value of y is {}", y);
    let five_hundred = tup.0; // access tuple element directly using index
    println!("The value of five_hundred is: {}", five_hundred);

    // array, allocated on stack
    let a = [1, 2, 3, 4, 5];
    /* arrays are useful when the size is unlikely to change
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
    // write an array's type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //initialize an array with the same value for each element
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

    // accessing an array
    let first = a[0];
    let second = a[1];
}
