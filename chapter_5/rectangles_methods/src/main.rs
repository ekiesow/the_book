#![allow(unused)]

// Methods are similar to functions:
// they're declared with the fun keyword and a name, can have params
// and a return value, and contain some code that runs when it's called.
// However, methods are defined within the context of a struct
// (or enum or trait object covered in chapters 6 and 17, respectively)
// and their first parameter is always self, which represent the instance of
// the struct the method is being called on.

fn main() {
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // The area function no longer takes in a Rectangle instance as a parameter
        // and is now a method defined on the Rectangle struct
        // We start with an impl (implementation) block to define the function
        // within the context of Rectangle. Then move the area function within the impl
        // block and the first parameter is now self and is used in place of the instance
        // in the body.
        impl Rectangle {
            // area method
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        // We use the method syntax to call the area function on our Rectangle instance
        // Rust knows the type of self is Rectangle due to the method being inside of
        // the impl Rectangle context.
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    // Methods can take ownership of self, borrow self immutably as we've done here,
    // or borrow self mutably, just as they can any other parameter.
    // We've chosen &self here as we don't take to take ownership and just wanted
    // to read from it, not write to it.
    // If we want to change the instance we'd use &mut self as the first parameter.

    // The main benefit of using methods is organization, we don't have to search
    // all over for capabilites of Rectangle. All things we can do with Rectangle
    // can be placed in one impl block.

    /* Methods with More Parameters */
    // We want to define a method in the impl Rectangle block named can_hold.
    // The function will take in another instance of Rectangle and return true,
    // if the second Rectangle can fit completely within self, otherwise returns false.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            // area method
            fn area(&self) -> u32 {
                self.width * self.height
            }

            // can_hold method
            // takes an immutable borrow of another Rectangle as a param
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    /* Associated Functions */
    // We are allowed to define functions within impl blocks that don't take
    // self as a parameter. These are called associated functions because
    // they're associated with the struct. They're still functions, not methods,
    // because they dont have an instance of the struct to work with.
    // String::from is an example of an associated function.

    // Associated functions are often used for constructors that will return
    // a new instance of the struct.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            // Associated function that makes it easier to create a square Rectangle
            // rather than having to specify the same value twice.
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        // To call this associated funtion we use the :: syntax with the struct name
        let sq = Rectangle::square(5);
    }

    /* Multiple impl Blocks */
    // Each struct is allowed to have multiple impl blocks which has each method
    // in its own impl block.
    // There's no rhyme or reason to separate these methods but its valid syntax!
    // In Chapter 10 we will see a case in which multiple impl blocks are useful
    // discussing generic types and traits.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
    }
}

/* Summary */
// Structs lets you create custom types that are meaningful for your domain.
// By using structs you can keep associated data connected and name each piece
// making code more clear. Methods let you specify behavior that instances of
// your struct have, and associated functions let you namespace functionality
// that is particular to your struct without having an instance available.
