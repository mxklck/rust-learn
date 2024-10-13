const THE_ANSWER: u32 = 42;

fn main() {
    println!("{}", THE_ANSWER);
    let mut x = 5;
    println!("x is {}", x);
    x = 17;
    println!("x now is {}", x);

    // also lets you re-type things
    // let spaces = "   ";
    // let spaces = spaces.len();

    let y = 5;
    let y = y + 1;
    {
        let y = y + 2; // this shadows by using let again (re-declaring), ends when scope ends!
        println!("y in inner scope is {}", y);
    }
    println!("y in outer scope is {}", y);

    let a = 5;
    let b = 3;
    let x = a / b;
    println!("{}", x);
    println!("{}", a % b);

    let my_tuple = (1, 3.4, "hello");
    // let (a, b, c) = my_tuple; // destructure the tuple
    println!("{}", my_tuple.2);

    let my_array: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.1]; // goes on stack, arrays have fixed length
    let easy_init_array = [0; 5];
    println!("{:?}, {}", my_array, my_array[2]); // pretty print the array!
    println!("{:?}", easy_init_array);
    some_fun_function();
}

fn some_fun_function() {
    let x = [1, 2, 3];
    println!("{:?}", x);
}
