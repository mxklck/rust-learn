use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Shhh, secret number is {}", secret_number);
    
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Guess should be an integer number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You got it!"),
        }
    }
}
