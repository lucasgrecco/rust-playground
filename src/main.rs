use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let error_message = "Failed to read line.";
    //let error_message_parse = "Please type a number";
    println!("The secret number is: {}", secret_number);

    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");
        io::stdin().read_line(&mut guess).expect(&error_message);

        //Parse input to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}
