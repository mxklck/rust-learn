use std::fmt::Display;
use std::str;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // we use string slices because we don't want to take ownership of the values
    // the signature expresses the following constraint:
    // that the returned reference will be valid as long as both the parameters are valid
    // The function signature also tells Rust that the string slice returned
    // from the function will live at least as long as lifetime 'a
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
lifetime conventions:
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str // lifetimes are a type of generics
where
    T: Display, //this is a trait bound (T needs to implement Display)
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let string3 = longest(&string1, &string2);
    println!("The longest string is: {}", string3);
    longest_with_an_announcement(&string1, &string2, "HELLO");
}
