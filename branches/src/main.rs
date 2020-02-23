fn main() {
    let number = 7;

    // blocks of code in if expressions are sometimes called arms
    // if number {     will fail, must have bool condition
    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false.");
    }

    if number != 0 {
        println!("number was something other than zero.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*
     * Because if is an expression, we can use it on the right side
     * of a let satement and numbers are expression themselves.
     * Both arms of the if must be the same type!
     */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is now : {}", number);
}
