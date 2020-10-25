#![allow(unused)]

// slice is a data type that does not have ownership
// Slices let you reference a contiguous sequence of elements in a collection

fn main() {
    /* The Slice Type */

    // function that takes a string and returns the index of the end of the first word
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // convert string to array of bytes

        // create iterator over array of bytes
        // enumerate creates a tuple: the first element is the index,
        // the second element is a reference to the element
        // the for loop destructures the tuple (index, reference to item)
        for (i, &item) in bytes.iter().enumerate() {
            // search for the byte that reperesents a space
            // using the byte literal syntax, b''
            if item == b' ' {
                // return the position if we find the space
                return i;
            }
        }

        // return the length of the string if no space is found
        s.len()
    }

    // the previous is error prone and tedious to use an index
    // the string s could be modified and then our index no longer points
    // to the end of a word in s
    {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
    }

    /* String Slices */
    // Note: This assumes ASCII values. String slice range indices must occur
    //   at valid UTF-8 character boundaries. E.g. the program will exit with
    //   an error if you attempt to slice in the middle of a multibyte character
    // A string slice is a reference to part of a String

    // We can create slices using a rance specifying [starting_index..ending_index]
    // where starting_index is the first position in the slice and
    // ending_index is one more than the last position in the slice
    {
        let s = String::from("hello world");

        // reference to a portion of the String
        let hello = &s[0..5];
        let world = &s[6..11];

        // with Rust's .. range syntax,
        // you can drop the starting_index if you want to start at 0
        let hello = &s[0..5];
        let hello = &s[..5]; // equivalent to the line above

        // if your slice includes the last byte of the String,
        // you can drop the trailing number
        let len = s.len();
        let world = &s[6..len];
        let world = &s[6..]; // equivalent to the line above

        // you can also drop both values if you want to take a slice of the entire string
        let hello_world = &s[0..len];
        let hello_world = &s[..]; // equivalent to the line above

        // rewrite first_word to return a slice
        // the type that signifies string slice is written as &str
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    // return a slice using the index of the end of the word
                    return &s[..i];
                }
            }
            &s[..] // return a slice of the whole string if no space is found
        }

        // returning a slice would also work for a second_word function
        // the compiler will make sure we have a valid reference to the String
        fn second_word(s: &String) -> &str {}

        // Slices make the previously mentioned bug impossible and the compiler will
        // let us know there is a problem as the references to the slice will be invalid
        let word = first_word(&s);
        s.clear(); // error!
        println!("the first word is: {}", word);
        // We cannot take a mutable reference of an immutable reference
        // because clear needs to truncate String, it needs to get a mutable reference
        // Rust does not allow this and compilation will fail
    }

    /* String Literals Are Slices! */
    // string literals are stored inside the binary
    let s = "Hello, world!";
    // the type of s here is &str: it's a slice pointing to
    // that specific point of the binary. This is why string literals are immutable
    // &str is an immutable reference

    /* String Slices as Paramters */
    // improving the first_word function!
    // updating the signature to take in &str as a parameter will allow us to use
    // the same funciton on both &String and &str values
    {
        fn first_word(s: &str) -> &str {}
        // if we have a string slice, we can pass that directly
        let my_string = String::from("hello world");

        // first_word works on slides of 'String's
        let word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        // first_word works on slides of string literals
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
    }

    /* Other Slices */
    // There are also general slice types too, not just for strings!
    {
        // consider this array:
        let a = [1, 2, 3, 4, 5];

        // We might want to reference part of this array
        let slice = &a[1..3];
        // this slice has the type &[i32] and works the same way string slices do,
        // by storing a reference to the first element and a length
    }

    /* Summary */
    // The concepts of ownership, borrowing, and slices ensure memory safety in Rust
    // programs at compile time. Having the owner of data automatically clean up data
    // when the owner goes out of scope means you don't have to write and debug extra code.
}
