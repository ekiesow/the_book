#![allow(unused)]

// Rules of References
// * At any given time, you can have either one mutable reference 
//      or any number of immutable references
// * References must always be valid

fn main() {
    // It's possible to return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // We still have to return the String to the callling funtion
    // so we can still use the String after the call to this function
    // because string was moved into calculate_length
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }


    // Taking ownership and then returning ownership can be tedious
    // Luckily, we have references!
    {
        let s1 = String::from("hello");

        let len = calculate_length_with_ref(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    // calculate_length_with_ref function has a reference to an object
    // as a parameter instead of taking ownership of the value
    fn calculate_length_with_ref(s: &String) -> usize { // s is a reference to a String
        s.len()
    } // here s goes out of scope, but it does not have any ownership so nothing happens

    // The ampersands in passing the variable &s1 and in the function &String
    // are references, and they allow you to refer to some value without
    // taking ownership of it

    // Note: the opposite of referencing, &, is the dereference operator, *

    // We call having references as function parameters borrowing
    // We can't try to modify something we are borrowing
    // Just as variables are immutable by default, so are references
    {
        let s = String::from("hello");
        change(&s);

        fn change(some_string: &String) {
            some_string.push_str(", world!");
        }
    }


    /* Mutable References */
    // Frist we change s to be mut, then create a mutable reference with &mut s
    // and accept a mutable reference with some_string: &mut String
    {
    let mut s = String::from("hello");
    change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world!");
        }
    }

    // Mutable references have one big restriction:
    // You can only have one mutable reference in a particular scope
    // The following code will fail
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s; // no no no

        println!("{}, {}", r1, r2);
    }

    // The benefit of this restriction is that Rust can prevent data races
    // at compile time which is similar to a race contition
    // and happens when these three conditions occur
    // * Two or more pointers access the same data at the same time
    // * At least one of the pointers is being used to write to the data
    // * There's no mechanism being used to synchronize access to the data

    // We can use curly braces to create a new scope, 
    // allowing for multiple mutable references
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference after no problem

        let r2 = &mut s; // yes yes yes
    }

    // A similar rule exists for combining mutable and immutable references
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no probs
        let r3 = &mut s; // WOAH WE GOT A BIG PROBLEM HERE

        println!("{}, {}, {}", r1, r2, r3);
    }

    // We also cannot have a mutable reference while we have an immutable one
    // Users of an immutable reference don't expect the values to suddenly change

    // The reference's scope starts from where it is introduced and continues
    // through until the last time the reference is used
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no probs
        println!("{}, {}", r1, r2);
        // r1 and r2 are no longer used, scope ends here

        let r3 = &mut s; // ah, we good here
        println!("{}", r3);
    }


    /* Dangling References */
    // A dangling pointer is a pointer that references a location in memory
    // that may have been given to someone else, by freeing some memory while
    // preserving a pointer to that memory
    // In Rust, the compiler guarantees there will be no dangling references

    // Try to create a dangling reference, will be prevented with a compile-time error:
    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from.
    {
        let reference_to_nothing = dangling();

        fn dangling() -> &String { // returns a reference to a String
            let s = String::from("hello"); // s is a new String
    
            &s // we return a reference to a String, s
        } // s goes out of scope, and is dropped. It's memory is gone. DANGER!
    }

    // The solution is to return the String directly
    // This works without any problems
    // Ownership is moved out and nothing is deallocated
    {
        fn dangling() -> String {
            let s = String::from("hello");

            s
        }
    }
}
