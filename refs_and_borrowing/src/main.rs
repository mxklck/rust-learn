/*
At any given time, you can have either one mutable reference or any number of immutable references.
*/

fn main() {
    let mut s = String::from("Hello!");
    let len = calculate_length(&s); // & creates reference
                                    // creating a reference is called "borrowing"
                                    // we're not allowed to modify something we have a reference to (by default)
    println!("The length of '{s}' is {len}.");
    change(&mut s);
    println!("{s}");

    let mut s1 = String::from("Hello string 1");
    let r1 = &mut s1; // cannot borrow s1 as a mutable reference more than once
                      // let r2 = &mut s1; // gives compile time error due to above

    let mut s2 = String::from("Hello string 2");
    let r1 = &s2; // const reference
    let r2 = &s2; // const reference, no problem
                  // let r3: = mut& s2; // problem
                  // -> cannot have a mutable reference while having immutable one to the same value.
                  // this is because users of the immutable reference do not expect the value to change

    /*
    Note that a reference’s scope starts from where it is introduced
    and continues through the last time that reference is used.
     */

    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point (their scope ends)

    let r3 = &mut s2; // now this is no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // reference does not transfer ownership, so s is still valid after call
    // references always point to valid data
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn change(some_string: &mut String) {
    // &mut creates a mutable reference
    some_string.push_str(" World?");
}
