use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // to run use cargo run -- the poem.txt
    // IGNORE_CASE=1 cargo run -- to poem.txt
    // main should be kept concise so it's easy to verify by inspection (as we can't unit test it)
    let args: Vec<String> = env::args().collect(); // collect() is used to convert the iterator into a collection
                                                   // unwrap or else deals with the result unwrapping.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // expect resolves the Result type and returns the value if it is Ok,
    // otherwise it panics with the message provided

    // we use if let rather than unwrap_or_else to handle the error
    // if error is returned it is bound to variable e.
    // we handle like this because we don't care about the value of the Ok variant
    // i.e. it's a unit type (void) and doesn't need to be parsed
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}"); // print to stderr
        process::exit(1);
    }
}
