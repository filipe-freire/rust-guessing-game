use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("🕹️  Let's play: Guess the number! 🕹️");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    game_loop(secret_number);
}

fn game_loop(secret_number: u32) {
    let mut attempts: u32 = 0;

    loop {
        if attempts >= 5 {
            println!("You ran out of attempts! Get good or die tryin' dude/dudette 😎");
            break;
        }

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Youuuu trickster! Type a number! >.<");
                increment_counter(&mut attempts);
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                increment_counter(&mut attempts);
            }
            Ordering::Greater => {
                println!("Too big!");
                increment_counter(&mut attempts);
            }
            Ordering::Equal => {
                println!("🕺🎈 You guessed it!! 🎈💃");
                break;
            }
        }
    }
}

fn increment_counter(attempts: &mut u32) {
    *attempts = *attempts + 1;
}
