use std::{cmp::Ordering, io::stdin};

use rand::random_range;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = random_range(0..=100);

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();
        stdin().read_line(&mut guess).expect("Failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        };
    }
}
