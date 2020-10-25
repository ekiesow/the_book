#![allow(unused)]
// Enums, or enumerations are useful and can be more appropriate than structs for cases
fn main() {
    // Say we are working with IP addresses. There are only two kinds: IPv4 and IPv6.
    // We can enumerate all possible variants. An IP address can only be one or the
    // other which makes the enum data structure appropriate.
    {
        // Both versions are fundamentally IP addresses so they should be treated the same
        // We define an IpAddrKind enumeration and listing the possible kinds they can be
        enum IpAddrKind {
            V4,
            V6,
        }

        /* Emnum Values */
        // We can create instances of each of the two variants of IpAddrKind:
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        // Note that the variants of the enum are namespaced under its identifier
        // We can then define a function that takes any IpAddrKind
        fn route(ip_kind: IpAddrKind) {}

        // We can call this function with either variant:
        route(four);
        route(six);

        // Using enums has even more advantages. At the moment we only know the type,
        // but don't have a wasy to store the actual IP address data. Given what we
        // know so far, we could tackle the problem like this:
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    // We can repeat this same concept in a more concise way using just an enum
    // We can attach data to an enum directly so there is no need for a struct
    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }

    // There's another advantage to an enum over a struct: each variant can have
    // different tupes and amounts of associated data.
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));
    }

    // As it turns out, encoding IP addresses is so common that the standard library
    // has a definition we can use. Let's look at how the standard library defines IpAddr
    struct Ipv4Addr {
        // -- snip --
    }

    struct Ipv6Addr {
        // -- snip --
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    // This illustrates that we can put any kind of data inside an enum variant.
    // Note we can still create our own definition of IpAddr even though the standard
    // library contains a definition.

    // Let's look at another example of an enum with a wide variety of types embedded
    enum Message {
        Quit,                       // no data associated at all
        Move { x: i32, y: i32 },    // Move includes an anonymous struct
        Write(String),              // Write includes a single String
        ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values
    }
    // This is similar to defining different kinds of struct definitions but
    // all the varients are grouped together under the Message type.

    // The following structs hold the same data that the preceding enum variants hold
    {
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct
    }
    // If we used different structs, we couldn't as easily define a function
    // to take any of these kinds of messages as we could with the Message enum

    // We are also able to define methods on enum with impl as we did with structs
    {
        impl Message {
            fn call(&self) {
                //method body calling self to get the value we called the method on
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    /* The Option Enum and Its Advantages Over Null Values */
    // The Option type encodes for the scenario in which a value could be something
    // or it could be nothing. This type system means the compiler can check whether
    // you've handled all the cases you should be handling. Therefore, Rust does not
    // have the null feature that many other languages have.
    // Inventor of null, Tony Hoare, called it his billion-dollar mistake.

    // Rust uses an enum that can encode the concept of a value being present or absent.
    // The Option<T> enum is defined by the standard template library as follows:
    {
        enum Option<T> {
            Some(T),
            None,
        }
    }

    // The Option<T> is so useful, it is even included in the prelude; you don't
    // need to bring it into scope explicitly. In addition, you can even use its
    // variants Some and None directly without the Option:: prefix.

    // The <T> syntax is a generic type parameter and will be coverd in Chapter 10.
    // For now, know that <T> means the Some variant of Option can hold one piece of
    // any data type.

    // Here are some examples of using Option values to hold number and string types
    {
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;
    }
    // If we use None rather than Some, we need to tell the Compiler what type of
    // Option<T> we have because it can't infer the type ahead of time.

    // Why is Option<T> better than having null?
    // Because Option<T> and T are different types, the compiler won't let us use
    // an Option<T> value as if it were definitely a value.

    // This code won't compile because it's trying to add an i8 to an Option<i8>
    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        let sum = x + y; // cannot add i8 with Option<i8>
    }

    // You have to convert an Option<T> to a T before you can perform T operations
    // with it. Generally, this helps catch one of the most common issues with null,
    // assuming that something isn't null when it actually is.

    // In order to have a value that can possibly be null,
    // you must explicitly opt in by making the type of that value Option<T>
    // Then, when you use that value, you are required to explicitly handle the case
    // when the value is null.

    // In general, in order to use an Option<T> value, you want to have code that
    // will handle each variant. You want some code that will run only when you have
    // a Somt(T) value and this code is allowed to use the inner T. You want some other
    // code to run if you have a None value, and that code doesn't have a T value available.
}
