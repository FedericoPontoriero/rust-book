// First hello world program and cargo setup

use std::io;

fn main() {
    println!("Hello, world!");
}

// Guessing game from chapter 2

fn guessing() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
