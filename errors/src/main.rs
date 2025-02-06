use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    // return the error
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error), // early exit
    };

    let mut username = String::new();
    
    // don't need to explicitly return the value as it is the last expression in the function
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
    // note that this always returns a member of the Result enum type! (OK(username) or Err(error))
}

fn better_read_username_from_file() -> Result<String, io::Error> {
    // key takeaway is that the ? operator performs an early exit with the error value
    // so it has to be used inside a function that returns a Result, 
    // Option or or another type that implements FromResidual
    let mut username_file = File::open("xyz.txt")?; // note the mut and the ? operator
    // if the Result is an Err, the ? operator will return the Err value early from the function
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    // if the Result is an Err, the ? operator will return the Err value from the function
    Ok(username) // again no semi-colon
}

fn even_better_read() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // or could use fs::read_to_string("hello.txt"), this is implemented as is very common
}

fn main() {
    // let v = vec![1, 2, 3]; // vec! is a macro that creates a Vec<T> with the elements you give it.
    // v[99]; // This will panic because the index is out of bounds.
    // this_one_panics();
    // Result is always in scope, so you can use it without the std:: prefix.
    // same goes for Option.

    // cleanest option is to use expect
    let best_result = even_better_read().expect("Failed to read username from file.");
    let better_user_result = better_read_username_from_file().expect("Failed to read username from file.");
    let user_result = read_username_from_file();
    let expect_result = File::open("hello.txt").expect("Expects a hello.txt file to exist.");

    // automatically unwraps result to get the value or panics if there is an error.
    let unwrap_result = File::open("hello.txt").unwrap();

    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            other_error => panic!("Problem opening the file: {other_error:?}"),
        },
    };

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
    // println!("{greeting_file:?}");
}

fn this_one_panics() {
    panic!("Crash and burn!!!");
}
