use std::vec;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // cmp::PartialOrd is a trait that allows us to compare values
    // <T:std::cmp::PartialOrd> is a trait bound that restricts the types of T to those that implement the PartialOrd trait
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x // returns a reference to x (a getter)
    }
}

// constraint methods that only apply when type of point is f32
impl Point<f32> {
    fn r(&self) -> f32 {
        // methods always take reference to self as first argument (I think)
        let r: f32 = (self.x.powi(2) + self.x.powi(2)).sqrt();
        r
    }
}

struct GenericPoint<T, U> {
    x: T,
    y: U,
}

// THERE IS NO RUN-TIME COST TO USING GENERICS :)! THEY ARE EXPANDED (monomorphized) at run time

fn main() {
    println!("Hello, world!");
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    // this is a reference to an element in numbers!
    println!("The largest number is {result}");

    let chars = vec!['a', 'b', 'g', 'c'];
    let largest_char = largest(&chars);
    println!("The largest char is {largest_char}");

    let int = Point { x: 5, y: 10 };
    let float = Point { x: 1.2, y: 4.0 };
    let mixed_point = GenericPoint { x: 'c', y: 11 };
    println!("int.x = {}, int.y = {}", int.x, int.y);
    println!("p.x = {}, r_is = {}", float.x(), float.r());
    println!("float.x = {}, float.y = {}", float.x, float.y);
    println!(
        "mixed_point.x = {}, mixed_point.y = {}",
        mixed_point.x, mixed_point.y
    );
}
