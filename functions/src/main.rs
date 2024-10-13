fn main() {
    println!("Hello, world!");
    let x: i64 = 44;
    some_function(x.try_into().unwrap(), [1, 2, 3]);
    another_function();
    let y = function_that_returns(5);
    println!("{y:?}");
    control_flow();
    loops();
}

fn some_function(x: i32, my_array: [i32; 3]) {
    println!("Here I am {x}! Also, {my_array:?}");
}

fn another_function() {
    let _y = 6; // this is a statement, does not return a value
                // new scopes are actually expressions (they return something!)
    let y = {
        let x = 3;
        x + 1 // expressions do not include ending semi-colons
              // semi colon is for statements (will not return a value)
    };
    println!("{y}");
}

fn function_that_returns(x: i32) -> i32 {
    return x + 1; // no semicolon! i.e. expression that returns
}

fn control_flow() {
    let n = 3;
    if n < 3 {
        println!("less than 3");
    } else if n == 3 {
        println!("number is 3");
    } else {
        println!("greater than 3");
    }
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break and return a value? why is there a semi-colon here?
        }
    }; // this semi colon ends the assignment to result
    println!("{result}");

    counter = 3;
    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // this is slow
    // "because the compiler adds runtime code to perform the conditional
    // check of whether the index is within the bounds of the array on every iteration through the loop."
    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }

    for element in a {
        println!("{element}");
    }
    // (1..4) is a range, it is inclusive of start but exclusive of end
    for number in (1..4).rev() {
        println!("Countdown number: {number}");
    }
}
