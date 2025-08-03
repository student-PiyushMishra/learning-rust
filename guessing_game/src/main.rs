use colored::Colorize;
use std::cmp::Ordering;
use rand;

fn main() {
    println!("{}","Welcome to The Guessing Game!!!".bold().cyan());
    let secret_number:u32 = rand::random_range(1..=100);
    let mut points_left:u8 = 15;
    loop{
        if points_left == 0 {
            println!("\n\nOut of points...");
            break;
        }
        println!("\n\n{}: {}","Points Left".yellow(),points_left);
        println!("Enter your guess: ");
        let mut guess:String = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Error while reading the user input...");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input");
                continue;
            }
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too Small.".red()),
            Ordering::Greater => println!("{}","Too Big.".red()),
            Ordering::Equal =>{ println!("{}","\nCorrect Guess.\n\n\nYou Win  !!!".green().bold()); break;}
            
        }
        points_left -= 1;
   }
}
