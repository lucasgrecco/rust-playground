use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut guess = String::new();
    let error_message = "Failed to read line.";

    io::stdin()
        .read_line(&mut guess)
        .expect(&error_message);

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);
}
