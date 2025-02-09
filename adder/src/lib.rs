use std::str;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: usize) -> usize {
    num + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*; //  brings in everything from the outer scope

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
    #[test] // attribute
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore] // ignores a test e.g. because it's slow
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 5,
            height: 10,
        };
        let smaller = Rectangle {
            width: 3,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_2() {
        let result = add_two(2);
        // note that using assert_eq! will require the #[derive(PartialEq, Debug)] 
        // traits to be implemented on your structs
        assert_eq!(result, 4);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name");
    }

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn test_guess() {
        Guess::new(200);
    }
}
