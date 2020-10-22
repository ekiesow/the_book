fn main() {
    /* Repeating Code with loop */

    /*
    // inf loop
    loop {
        println!("again!");
    }
    */

    // use the break keyword to tell the program when to exit a loop

    /* Returning Values from Loops */

    let mut counter = 0;

    let result = loop {
        // error by trial, cannot use ++, is not an expression
        counter += 1;
        if counter == 10 {
            break counter * 2; // value is returned
        }
    }; // semic ends statement that assigns value to result

    println!("The result is {}", result);
    println!("");

    /* Conditional Loops with while */

    // attempt with a loop
    println!("Commencing countdown with loop");
    let mut number = 3;
    loop {
        if number > 0 {
            println!("{}!", number);
            number -= 1;
        } else {
            break;
        }
    }
    println!("WE HAVE LIFTOFF!");
    println!("");

    // now with while
    println!("Commencing countdown with while");
    number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("WE HAVE LIFTOFF!");
    println!("");

    /* Looping Through a Collection with for */

    /*
     * first with while
     * this approach is error prone and slow as the compiler add runtime
     * code to perform the conditional check on every element on every
     * iteration through the loop
     */
    let a = [3, 5, 9, 11, 13];
    let mut index = 0;

    println!("print array with while");
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("");

    /*
     * now with a for loop
     * increased safety of the code and eliminated the chance of bugs
     * that might result from going beyond the end of the array or
     * missing items in the array.
     */
    println!("print array with for");
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    println!("");

    // rewrite the countdown loop to be safe using a range and reverse
    println!("Commencing countdown with for");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("WE HAVE LIFTOFF!");

    /* for loops in Rust are awesome! */
}
