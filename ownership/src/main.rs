fn main() {
    // string literal (fixed length, stack)
    let string_type_one = "hello";
    println!("{string_type_one}");

    // stored on heap, can change size :: can be used to namespace functions
    let mut string_type_two = String::from(string_type_one);
    string_type_two.push_str(", World!");
    println!("{string_type_two}");
}
