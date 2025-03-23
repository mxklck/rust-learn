#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    println!("Hello, world!");
    let b = Box::new(50); // box is stored on stack, but data is now stored on heap
    let a = *b; // dereference the box to get the value
    println! {"a = {a}"};
    println! {"b = {b}"};

    // enabling recursive types
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    // Box<T> implements the deref trait, so we can use * operator to dereference it
}
