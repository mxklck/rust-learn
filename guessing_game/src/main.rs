use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number!");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Shhh, secret number is {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
