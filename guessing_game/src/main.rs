use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("\x1b[46mGuess the number!\x1b[0m");

    //println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(res) => res,
            Err(_) => {
                println!("\x1b[91mPlease enter a number\x1b[0m");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
             println!("\x1b[92mYou win!\x1b[0m");
             break;
            }
        }
    }
}

