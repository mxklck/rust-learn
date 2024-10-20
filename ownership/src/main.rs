fn main() {
    // string literal (fixed length, stack)
    let string_type_one = "hello";
    println!("{string_type_one}");

    {
        // stored on heap, can change size :: can be used to namespace functions
        let mut string_type_two = String::from(string_type_one);
        string_type_two.push_str(", world!");
        println!("{string_type_two}");
    } // drop function called on string when curly bracket is hit, memory is freed up

    let x = 5;
    let y = x;
    // because they're both simple ints (on stack) y is a copy of value in x (not a reference)
    println!("{x}, {y}");

    let s1 = String::from("hello"); // is a pointer to a vector
                                    // len is bytes currently used,
                                    // capacity is total bytes that string has received from allocator

    let s2 = s1;
    // when doing this assignment, we copy the pointer, the len and capacity
    // i.e. owen only copy the things on the stack, not on the heap
    // this is called a move
    // s1 was moved into s2 and s1 is no longer valid

    let s3 = s2.clone();
    // clone "deeply" copies the data; i.e. data on the stack and the heap is copied
    // this will be expensive (arbitrary code is executed)

    // only variables that are stored on the heap are moved. e.g. int is not

    println!("{s2}, {s3}"); // -> borrow of moved value, it's deleted s1 and it's no longer valid!

    ownership_example();
}

fn ownership_example() {
    let s = String::from("Hello, ownership example!");
    take_ownership(s);
    // println!("{s}"); // compile error as ownership has moved!
    
    let x = 4;
    make_copy(x);
    println!("{x}"); // this is fine

    let s1 = give_ownership();
    println!("{s1}");
    let s3 = take_and_give_back(s1); // s1 has been moved!
    println!("{s3}");
    // println!("{s1}"); // raises error
    let (s3, length) =  calculate_length(s3);
    println!("String {s3} has length {length}");
}

fn give_ownership() -> String {
    let some_string = String::from("I've come from give_ownership");
    some_string // no semi-colon (returns)
}

fn take_and_give_back(some_string: String) -> String {
    some_string
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}")
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed
 
fn make_copy(some_int: i32) { // some_integer comes into scope (as a copy)
    println!("{some_int}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}