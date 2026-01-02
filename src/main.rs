use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number: I've chosen a number between 1 and 100.\nPlease enter your guess:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        if let Err(err) = io::stdin().read_line(&mut guess) {
            eprintln!("Failed to read input: {err}");
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer (e.g. 42).");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("Please enter a number between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again:"),
            Ordering::Greater => println!("Too big! Try again:"),
            Ordering::Equal => {
                println!("You guessed it! The number was {secret_number}.");
                break;
            }
        }
    }
}
