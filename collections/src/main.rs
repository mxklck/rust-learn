/* all items from collections are variable length and stored on heap */
#[derive(Debug)]

// this is a workaround that lets you hold different data-types in a vector!
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vector memory is allocated contiguously
    let mut v: Vec<i32> = Vec::new();
    println!("{v:?}");
    v.push(5); // push i.e. push_back C++
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v { // get an immutable reference to iterate over
        println!("{i}");
    }

    // or, if you need to change the values in your vector...
    let mut v = vec![100, 32, 57];
    for i in &mut v{
        *i += 50; // why do I need to de-reference this? so I can get to the value
    }

    println!("{v:?}");

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.4),
        SpreadSheetCell::Text(String::from("This is a row")),
    ];

    println!("{row:?}");

    let third = &v[2]; // indexing (get's a reference, seems to be slice?)
    println!("{third}");

    let third = v.get(2);  // returns option as can fail to access
    match third {
        Some(x) => println!("The third element is {x}"),
        None => println!("There is no third element")
    }
    println!("{third:?}");



    println!("{v:?}");
     // vec! is a macro to initialize with an array
    let initialized_vector = vec![1, 2, 3];
    println!("{initialized_vector:?}");
}
