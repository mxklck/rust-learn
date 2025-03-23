use std::ops::Deref;

use crate::List::{Cons, Nil};
#[derive(Debug)]
struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

// trait implementations
impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type

    // behind the scenes, rust actually runs *(y.deref())
    fn deref(&self) -> &Self::Target {
        &self.0 // returns a reference to the first element of the tuple struct
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with value {}", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = &x; // reference to a value
    let y_box = Box::new(x);
    let y_my_box = MyBox::new("Jim");

    println!("{y_my_box:?}");
    assert_eq!(5, x);
    assert_eq!(5, *y); // * is the dereference operator
    assert_eq!(*y, *y_box);
    assert_eq!("Jim", *y_my_box);

    let message = MyBox::new(String::from("AI Overlord"));
    hello(&message); // String is coerced into &str

    let _a = CustomSmartPointer {
        data: String::from("MY A STRING!"),
    };

    let _b = CustomSmartPointer {
        data: String::from("MY B STRING!"),
    };
    // std::mem::drop is automatically in the prelude
    drop(_a); // manually drops
              // variables are dropped in reverse order of creation when leaving a scope on }

    /*Multiple Ownership with reference counted smart pointers (Rc<T>) */
    // Rc<T> keeps track of how many references to a value there are
    // Rc<T> is only for use in single-threaded programs

}


enum List {
    Cons(i32, Box<List>),
    Nil,
}
