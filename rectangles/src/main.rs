#[derive(Debug)] // adding this outer attribute adds debug printing functionality
struct Rectangle {
    width: u32,
    height: u32,
}
// this is an "implementation block"
impl Rectangle {
    // all functions within the impl block are called "associated functions"

    // associated functions that aren’t methods (no reference to self)
    // are often used for constructors that will return a new instance of the struct, e.g.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        // borrows the self instance (read only as no mut)
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

fn main() {
    let square = Rectangle::square(5); // calling associated functions
    // (this is essentially) class methods vs instance methods
    
    println!("{}", square.area());

    let my_rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", my_rectangle.area());

    let my_second_rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let my_third_rectangle = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "Can my_rectangle hold? {}",
        my_rectangle.can_hold(&my_second_rectangle)
    );
    println!(
        "Can my_rectangle hold? {}",
        my_rectangle.can_hold(&my_third_rectangle)
    );
}
