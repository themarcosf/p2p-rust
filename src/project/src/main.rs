use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let _ = io::stdin().read_line(&mut guess);

    println!("You guessed: {guess}");

    let secret_number = rand::rng().random_range(1..=10);

    if guess.trim().parse::<u32>().unwrap() == secret_number {
        println!("You win!");
    } else {
        println!("You lose!");
    }

    println!("The secret number is: {secret_number}");
}
