use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\n\nGuess the number!");

    let secret = rand::rng().random_range(1..=100);

    let mut times_left = 15;

    loop {
        if times_left == 0 {
            println!("\nOut of times! You lost, the correct guess was {}.",secret);
            break;
        }
        println!("Please enter your guess:-");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        times_left = times_left - 1;

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            },
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Equal => {
                println!("\nCorrect Guess! WON with {} chances left.",times_left);
                break;
            },
            Ordering::Greater => {
                println!("Too big!");
            },
        };

        println!("\nTimes left: {}",times_left);
    }
}
