use std::io;
use rand::Rng;

fn main() {
    println!("Try to Guess the correct number as fast as possible!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 0;

    loop {
        num_guesses += 1;

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Your Guess Too small!"),
            std::cmp::Ordering::Greater => println!("Your Guess Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! It took you {} guesses.", num_guesses);
                break;
            }
        }
    }
}
