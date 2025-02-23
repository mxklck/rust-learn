use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // new is expected to never fail, so we rename to build
    // 'static is a lifetime specifier,
    // which means that the reference will live for the entire duration of the program
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        // clone is used to create a new string from the reference
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}

// not sure what this does?
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // config is moved, not borrowed!
    // the question mark operator is used to propagate errors up the call stack
    // it is a shorthand for unwrapping the result and returning the error if it is an Err
    let contents = fs::read_to_string(&config.file_path)?;
    println!("Contents of the file are: \n{}", contents);
    Ok(())
    // () is the unit type, which is a type that has only one value: an empty tuple
    // it is used when we need to return a value but don't have anything meaningful to return
    // it is similar to void in C and C++
    // Box<dyn Error> is a trait object, which means it can hold any type that implements the Error trait
    // dyn is short for dynamic, which means that the type of the value is determined at runtime
    // Box is a smart pointer that points to heap-allocated memory
    // it is used to store data when we don't know the size of the data at compile time
    // Box<dyn Error> returns a type that implements the Error trait
}

fn main() {
    // to run use cargo run -- the poem.txt
    // main should be kept concise so it's easy to verify by inspection (as we can't unit test it)
    let args: Vec<String> = env::args().collect(); // collect() is used to convert the iterator into a collection
                                                   // unwrap or else deals with the result unwrapping.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // expect resolves the Result type and returns the value if it is Ok,
    // otherwise it panics with the message provided
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // we use if let rather than unwrap_or_else to handle the error
    // if error is returned it is bound to variable e.
    // we handle like this because we don't care about the value of the Ok variant
    // i.e. it's a unit type (void) and doesn't need to be parsed
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
