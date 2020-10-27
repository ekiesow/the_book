#![allow(unused)]

// Rust has an extremely powerful control flow operator called match that allows
// you to compare a value against a series of patterns and then execute code based
// on which pattern matches. The power comes from the expressiveness of the patterns
// and the fact that the compiler confirms that all possible cases are handled.

fn main() {
    /* The match Control Flow Operator */
    // We can write a function that can take an unkown US coin and determine
    // which coin it is and return its value in cents.
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        // We list the match keyword followed by an expression, which in this case
        // is the value coin. This seems similar to an if but does not need to
        // return a boolean value and can be any type.

        // Next there are match arms having two parts: a pattern and some code.
        // The first pattern is Coin::Penny and then => operator followed by the
        // code to run. Match compares the resulting value against the pattern of
        // each arm in order then running the code associated with the pattern.

        // The code associated with each arm is an expression and the resulting
        // value of the expression is the value that gets returned for the entire
        // match expression.
    }

    // If you want to run multiple lines of code in a match arm, you can use curly
    // brackets.
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        // We want to print "Lucky penny!" every time the the method was called
        // with a Coin::Penny and still return the last value of the block, 1.
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    // Another useful feature of match arms is taht they can bind to the parts
    // of the values that match the pattern. This is how we can extract values
    // out of enum variants.
    {
        // From 1999 through 2008, the United States minted quarters with
        // different designs for each of the 50 states on one side.
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        // In the match expression we add the variable called state to the pattern
        // that matches values of the variant Coin::Quarter. When a Coin::Quarter
        // matches, the state variable will bind to the value of that quarter's state.
        // Then we cas use state in the code for that arm, like so.
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                // If we pass in Coin::Quarter(UsState::Alaska) for coin,
                // the binding for state will be the value UsState::Alaska.
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
    }

    /* Matching with Option<T> */
    // In the previous section, we watned to get the inner T value out of the Some
    // case when using Otpion<T>; We can also handle Option<T> using match by
    // comparing the variants of Option<T>. Let's say we want to write a function
    // that takes an Option<i32> and, if there's a value inside, add 1 to that value.
    // If there isn't a value inside, return the None value instead.
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        // When we call plus_one(five), the value x in the body of plus_one will
        // have the value Some(5). This matches the Some(i) pattern and i binds to
        // the value contained in some, 5. The code in the arm executes and creates
        // a new Some value with our total 6 inside.
    }

    /* Matches Are Exhaustive */
    // Consider this version of our plus_one function that has a bug and won’t compile:
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
            }
        }
    }
    // We didn't handle the None case and Rust knows we didn't cover every possible
    // case. Matches in Rust are exhaustive: we must exhaust every last possibiltiy
    // in order for the code to be valid, especially in the case of Option<T>.

    /* The _ Placeholder */
    // Rust has a pattern we can use when we don't want to lista ll possible values.
    {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }
    // The _ pattern will match any value and the () is just a unit value,
    // so nothing will happen in the _ case.
}
