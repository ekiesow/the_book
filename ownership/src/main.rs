#![allow(unused)]

/*
 * Ownership Rules:
 * Each value in Rust has a variable that's called its owner
 * There can only be one owner at a time
 * When the owner goes out of scope, the value will be dropped
 */

fn main() {
    /* Variable Scope */
    // scope is the range that the variable is valid
    {
      let s = "hello"; // s refers to a string literal
      // do stuff with s

    } // s is out of scope here


    /* String Type */
    // String data type is stored on the heap
    // the size of a string may be unkown at compile time

    // can create a string from a string literal using the from function
    let s = String::from("hello");
    // this kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'


    /* Memory and Allocation */
    // string literals are fast and efficient due to their immutability
    // the text is hardcoded and we know the contents at compile time

    // With the String type, we need to allocate an amount of memory on the heap
    // * The memory must be requested form the memory allocator at runtime
    // * We need a way of returning this memory to the allocator when we're done
    //      with our String

    // There is no garbage collector in Rust keeping track of and cleaning up memory
    // The responsibility as the programmer to return memory when it is no longer
    //    being used has been a historically difficult problem

    // Instead, the memory is automatically returned once the variable that owns
    // it goes out of scope
    {
        let s = String::from("hello");
        // do stuff with s
    } // scope is now over, memory is now returned

    // When a variable goes out of scope, Rust calls a special function for us
    // This function is called drop, and is called automatically at the closing }


    /* Ways Variables and Data Interact: Move */
    // A String is made up of three parts stored on the stack:
    // A pointer to memory, a length, and capacity
    let s1 = String::from("hello");
    let s2 = s1;
    // When we assign s1 to s2, the String data is copied including the pointer,
    // length, and capacity.
    // The data on the heap that the pointer refers to is not copied

    // Because the drop function cleans up the heap memory
    // s1 is no longer considered to be valid in order to ensure memory safety
    // drop will only be called when s2 goes out of scope
    println!("{}, world!", s1); // this won't work!
    // The first variable is invalidated so the shallow copy
    //     is instead known as a move

    // Rust will never create "deep" copies of your data
    // therefore any automatic copying can be assumed to be inexpensive
    // in terms of runtime performance


    /* Ways Variables and Data Interact: Clone */
    // We can use a method called clone if we do want to deeply copy the heap
    // data of the String
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /* Stack-Only Data: Copy */
    // Here we do not have to call clone yet x is valid and wasn't moved into y
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // Integers have a known size at compile time and are entirely on the stack
    // This makes copying quick and easy

    // Rust has a special annotation called the Copy trait
    // If a type has a Copy trait, the older variable is still valid after assignment

    // As a general rule, any group of simple scalar values can be Copy,
    // and nothing that requires allocation or is a form of resource are Copy
    // Types that are Copy:
    // * All integer types such as u32
    // * The Boolean type, bool
    // * All floating point types such as f64
    // * The character type, char
    // * Tuples, if they only contain types that are also Copy


    /* Ownership and Functions */
    // Passing a variable to a function will move or copy, just as assignment does
    {
        let s = String::from("hello");  // s comes into scope
    
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
    
        let x = 5;                      // x comes into scope
    
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use x afterward
    
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
    
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    // If we tried to use s after the call to takes_ownership,
    // Rust would throw a compile time error


    /* Return Values and Scope */
    // Returning values can also transfer ownership
    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
      // moved, so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {             // gives_ownership will move its
                                                 // return value into the function
                                                 // that calls it

        let some_string = String::from("hello"); // some_string comes into scope

        some_string                              // some_string is returned and
                                                 // moves out to the calling
                                                 // function
    }

    // takes_and_gives_back will take a String and return one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                          // scope

        a_string  // a_string is returned and moves out to the calling function
    }
}
