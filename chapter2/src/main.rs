// docs at https://doc.rust-lang.org/std/io/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let rand_num = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {rand_num}");

    loop {
        println!("Please input your guess.");

        // mutable String builder (not str)
        let mut guess = String::new();

        // need expect for Err handling. guess is modified as side effect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing. sequentially?
        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
