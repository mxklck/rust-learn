pub mod garden;

// use garden::vegetables::Asparagus;
// same as 
use crate::garden::vegetables::Asparagus; // create acts as self / .

fn main() {
    println!("Hello, world!");
    let vegetable = Asparagus {
        color: String::from("green"),
    };
    println!("{vegetable:#?}");
}
