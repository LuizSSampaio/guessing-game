use rand::{thread_rng, Rng};
use std::io;

fn main() {
    println!("=== STARTING THE GUESSING GAME ===");
    println!("Choose a number between 1 and 10");

    let mut random = thread_rng();
    let random_number: u8 = random.gen_range(1..=10);
    let mut guess_string = String::new();

    print!("Write your guess: ");
    io::stdin()
        .read_line(&mut guess_string)
        .expect("Write an valid guess next time");

    let guess_number = guess_string.trim().parse();
    match guess_number {
        Ok(number) => {
            println!("Your guess is {}", guess_string);
            println!("My guess is {}", random_number);

            if random_number == number {
                println!("You won!");
            } else {
                println!("You lose bitch!");
            }
        }
        Err(_) => {
            println!("Write an valid guess next time");
        }
    }
}
