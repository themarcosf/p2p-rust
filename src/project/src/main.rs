use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let _ = io::stdin().read_line(&mut guess);

    println!("You guessed: {guess}");
}
