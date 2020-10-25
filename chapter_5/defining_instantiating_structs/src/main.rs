#![allow(unused)]

/* Defining and Instantiating Structs */
// Structs are similar to tuples as the pieces of a struct can be different types
// but you'll name each piece of data so it's clear what the values mean
// structs are more flexible as you don't have to rely on the order of data
// to specify or access the values of an instance

// Use the struct keyword to define a struct and name the entire struct
// Inside the curly braces we define the names and types of the pieces of data
// called fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // To use a struct after we've defined it,
    // we create an instance of that struct by specifying concrete values
    // for each of the fields.
    // We create an instance by stating the name of the struct and then add
    // curly brackets containg key: value pairs.
    // We do not need to specify the fields in the same order in which they are
    // declared in the struct
    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        // To get a specific value from a struct, we can use dot notation.
        // If we want want this user's email we can use user1.email wherever we need it
        let email = user1.email;
    }

    // If we make this instance mutable, we can change a value using dot notation
    // Note that the entire instance is mutable, Rust doesn't allow only certain
    // fields to be mutable
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        // change the value of a mutable User instance
        user1.email = String::from("someoneelse@amazon.com");

        // build_user function returns a User instance with the given email and username
        // the active field gets the value of true and sign_in_count a value of 1
        fn build_user(email: String, username: String) -> User {
            User {
                email: email,
                username: username,
                active: true,
                sign_in_count: 1,
            }
        }
    }

    /* Using the Feild Shorthand when Variables and Fields Have the Same Name */
    // Because the parameter names and the struct field names are exactly the same
    // we can use the field init shorthand syntax to rewrite build_user
    {
        fn build_user(email: String, username: String) -> User {
            User {
                email,    // sets the email field to the value in the email parameter
                username, // sets the username field to the value in the username parameter
                active: true,
                sign_in_count: 1,
            }
        }
    }

    /* Creating Instances From Other Instances With Struct Update Syntax */
    // It's ofen useful to create a new instance of a struct that uses most
    // of an old instance's values but changes some. We will do this using
    // struct update syntax
    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        // create user2 from user1 without update syntax updating only email and username
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        };

        // The struct update syntax .. lets us specify that the remaining fields
        // not explicitly set should have the same value as the fields in the
        // given instance
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }

    /* Using Tuple Structs without Named Fields to Create Different Types */
    // You can also define structs that look similar to tuples, called tuple structs
    // Tuple structs have the added meaning the struct name provides but don't have
    // names for their fields but rather just the types.
    // This is useful when you want to give the whole tuple a name and make the tuple
    // be a different type from other tuples and naming each field would be redundant
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        // Note that black and origin are different types
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        // You can destructure tuple structs like tuples into their individual pieces,
        // and you can use a . followed by the index to access and individual value, etc
        let Point(x, y, z) = origin;
        let first = x;
        let x = origin.0;
    }

    /* Unit-Like Structs Without Any Fields */
    // You can also define structs that don't have any fields!
    // These are called unit-like structs because they behave similarly to (),
    // the unit type. These can be useful in situations where you need to implement
    // a trait on some type but don't have any data that you want to store in the
    // type itself. Traits will be discussed in chapter 10.

    /* Ownership of Struct Data */
    // We cannot store a reference to data owned by something else
    // without the use of lifetimes. Othersie we must use owned types
    // such as String instead of references like &str.
    // Lifetimes will be discussed in chapter 10.
    {
        struct User {
            username: &str,
            email: &str,
            sign_in_count: u64,
            active: bool,
        }

        let user1 = User {
            email: "someone@example.com", // must have lifetime to store a ref or be an owned type
            username: "someusername123",  // must have lifetime to store a ref or be an owned type
            active: true,
            sign_in_count: 1,
        };
    }
}
