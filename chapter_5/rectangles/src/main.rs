// Program that calculates the area of a rectangle
fn main() {
    // We'll start with single variables, then refactor to use structs instead
    {
        // height and width specified in pixels
        let width1 = 30;
        let height1 = 50;
        // calculate the area of a rectangle
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }

    /* Refactoring with Tuples */
    // The area function has two parameters but we can do better!
    // height and width are related because they describe one rectangle
    // Let's express this using tuples
    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }
    // Tuples let us add a bit of structure and now we're passing one argument
    // but this version is less clear since tuples don't have named elements
    // If someone were looking at this code and needed the width,
    // they wouldn't know which is which or would have to keep track

    /* Refactoring with Structs: Adding More Meaning */
    // We use structs to add meaning by labeling data.
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }
    // Here we defined a struct named Rectangle and inside of the curly brackets,
    // we defined with fields width and height of type u32.
    // In main we created an instance of Rectangle with width of 30 and height of 50.
    // Our area function is defined with one parameter, which we named rectangle,
    // whose type is an immutable borrow of a struct Rectangle instance.
    // We want to borrow the struct rather than take ownership of it that way main
    // retains ownership of it and can continue using rect1.
    // The return in the area function is more descriptive rather than using the
    // tuples' 0 and 1 indexing

    /* Adding Useful Functionality with Derived Traits */
    // The macro println uses formatting known as Display that can do many kinds
    // of formatting.
    // structs don't have an implementation of Display due to ambiguity
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {}", rect1); // won't work!!!
    }

    // When debugging, we can use the specifier :? in the curly brackets to use
    // an output format called Debug. This allows us to print our structs in a way
    // that is useful for developers so we can see its value while we're debugging
    // our code. We have to explicitly opt in to make the functionality available
    // to our struct using the annotation #[derive(Debug)] just before the struct
    // definition.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {:?}", rect1);
        println!("rect1 is {:#?}", rect1); // pretty-print
    }
    // Rust provides a number of traits for us to use with the derive annotation
    // that will be covered, as well as creating your own traits, in chapter 10.
}
