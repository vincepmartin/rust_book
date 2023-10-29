// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
        
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..=100);
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert our String to a number in a safe way.
        // Since guess is mutable we can just change its value as such.
        // Also, we use a match statement to handle the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check if we have a correct match.
        // Here we use match instead of if/else etc.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        println!("You guessed: {guess}");
    }
}
