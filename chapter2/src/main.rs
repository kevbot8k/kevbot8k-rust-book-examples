// docs at https://doc.rust-lang.org/std/io/
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mutable String builder (not str)
    let mut guess = String::new();

    // need expect for Err handling
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
