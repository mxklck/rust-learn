use std::collections::HashMap;

fn main() {
    //Just like vectors, hash maps store their data on the heap. (they change size)
    let mut scores = HashMap::new(); // new makes new empty object
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");

    // overwriting values
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // adding only if key is not present
    scores.entry(String::from("Blue")).or_insert(100); // doesn't update
    scores.entry(String::from("Red")).or_insert(101);
    println!("{scores:?}");

    // updating a value based on an old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or inset returns a mutable reference to the *value* of the entry
        let count = map.entry(word).or_insert(0);
        *count += 1; // dereference count as + is not defined on reference
    }

    println!("{map:?}");

    let team_name = String::from("Blue");
    // this is a lot of syntax to access one element...
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get returns an Option (enum{Some, None}) if no value it returns None
    // copied maps an Option<&T> to an Option<T> by copying the contents of the option.
    // unwrap_or sets score to zero if not found.
    // makes sense :)
    println!("{team_name} has {score} points.");

    // can loop through reference (borrowed)
    // note that this will print in an **arbitrary** order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /*
    For types that implement the Copy trait, like i32, the values are copied into the hash map.
    For owned values like String, the values will be moved and the hash map
    will be the owner of those values, as demonstrated below:
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{map:?}");
    // field_name and field_value are invalid at this point as they've been moved
    // e.g. println!("{field_name}"); won't work

    let mut my_vector = vec![2, 1, 3, 5, 5, 4, 5];
    println!("Input vector: {my_vector:?}");
    let (median_value, mode_value, mean_value) = vector_averages(&mut my_vector);
    println!("median: {median_value}");
    println!("mode: {mode_value}");
    println!("mean: {mean_value}");
}

// v is already a reference, so no references needed here?
// not too sure how this works to be honest...
fn vector_averages(v: &Vec<i32>) -> (i32, i32, f64) {
    let median_value = median(v);
    let mode_value = mode(v);
    let mean_value = mean(v);
    (median_value, mode_value, mean_value)
}

fn median(v: &Vec<i32>) -> i32 {
    // pass reference as I do not want to take ownership
    let mut vec = v.clone();
    vec.sort();
    let mid_index = (vec.len() - 1) / 2; // not doing rounding etc...
    vec[mid_index]
}

fn mode(v: &Vec<i32>) -> i32 {
    // mode will be the maximum count
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in v {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max = 0;
    for (key, value) in &counts {
        if *value >= max {
            max = *value;
            mode = *key;
        }
    }
    mode
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut count = 0;
    for i in v {
        sum += i;
        count += 1;
    }
    let mean = f64::from(sum) / f64::from(count);
    mean
}
