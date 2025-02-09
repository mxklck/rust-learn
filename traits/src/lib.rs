use std::fmt::Display;

pub trait Summary {
    // this is a trait?
    fn summarize(&self) -> String; // note the semicolon here instead of providing an implementation

    fn read_more(&self) -> String {
        // this is a default implementation
        format!("Read more... from {}", self.summarize_author()) // format macro concatenates strings
    }

    fn summarize_author(&self) -> String;
}
// each type implementing the Summary trait must provide its own implementation of the summarize method
// this is a bit like headers in C
// i guess you can use this to build object like interfaces

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // implement the Summary trait for the NewsArticle struct
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // implement the Summary trait for the Tweet struct
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    // this function takes a parameter of any type that implements the Summary trait
    // this is slightly confusing
    // basically, anything that implements the Synopsis trait can be passed to this function
    println!("Breaking news! {}", item.summarize());
    println!("Brought to you by: {}", item.summarize_author());
}

// this defines a generic struct
struct Pair<T> {
    // a struct with two members of the same type T, e.g. Pair<i32>
    x: T,
    y: T,
}

// this implements a method for the Pair struct
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
        // Self is a type alias for the Pair struct
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // this will only be implemented for types that implement the Display and PartialOrd traits
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        let news = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };
        println!("1 new tweet: {}", tweet.summarize());
        println!("Read more: {}", tweet.read_more());
        println!("Read more: {}", news.read_more());
        notify(&news);
        assert!(true);
    }

    #[test]
    fn test_pairs() {
        let pair = Pair::new(5.2, 10.1);
        pair.cmp_display();
        assert_eq!(pair.x, 5.2);
        assert_eq!(pair.y, 10.1);
    }
}
