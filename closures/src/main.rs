use std::{thread, time::Duration, vec};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else takes one argument; a closure that returns a value of the type stored in the Some variant
        user_preference.unwrap_or_else(|| self.most_stocked())
        // if Option<T> is Some variant, it returns the value inside the Some variant
        // if it is the None variant, it calls the closure and returns the value returned by the closure
        /*
        || self.most_stocked() <- defines a closure that takes no arguments and returns the value returned by self.most_stocked()
        parameters of closures appear between the vertical bars (||)

        The closure captures an immutable reference to self, which is the Inventory instance
        functions cant capture the environment in this way
        */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}

fn explore_threads() {
    let list = vec![1, 2, 3];
    println!("Before defining the closure: {list:?}");

    // move forces the closure to take ownership of the values in its environment
    thread::spawn(move || println!("From thread: {list:?}")).join(). unwrap();

}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    // remember Optional enum has two variants: Some and None
    let user_preference1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_preference1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_preference1, giveaway1
    );

    let user_preference2 = None;
    let giveaway2 = store.giveaway(user_preference2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_preference2, giveaway2
    );

    // so closures are like functions that can be stored in variables
    // it has access to the environment in which it was created
    let expensive_closure = |num: i32| -> i32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(0));
        num + 1
    };

    let x = 0;
    let y = expensive_closure(x);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let mut list = vec![1, 2, 3];
    println!("Before: {list:?}");

    // this borrows the list (an immutable reference to the list)
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After  calling closure: {list:?}");

    let mut mutably_borrows = || list.push(7); // <- needs mut keyword to borrow the list mutably

    mutably_borrows();
    println!("After calling the borrowing closure: {list:?}");

    explore_threads();
}
