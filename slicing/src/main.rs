/*A slice is a type of reference, so it does not take ownership */
fn main() {
    let my_string = String::from("Hello, World!");
    let word = first_word(&my_string); // this is annoying, hence we have string slices!
    let hello = &my_string[0..5]; // slice syntax = [starting_index..ending_index]
    let world = &my_string[7..12]; // [i..j] is a range
    println!("{word}, {hello}, {world}");
    // with the range syntax [..2] is equal to [0..2]
    // also [3..len] is equal to [3..]
    // [..] takes the entire string
    // my_string.clear(); <- does not work now
    // println!("{word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{a:?}");
    println!("{slice:?}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // array of bytes (arrays are fixed length) []
    for (i, &item) in bytes.iter().enumerate() {
        // get a reference
        if item == b' ' {
            return &s[0..i]; // if it finds a space return the index
        }
    }
    &s[..] // otherwise return the index of the end
}
