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
    // len is bytes currently used, capacity is total bytes that string has received from allocator
    let s2 = s1; // when doing this assignment, we copy the pointer, the len and capacity
    // i.e. owe only copy the things on the stack, not on the heap
    
    // this is a move (s1 was moved into s2, as s1 is no longer valid)
    println!("{s1}"); // -> borrow of moved value, it's deleted s1 and it's no longer valid!
}
