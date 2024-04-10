use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("{}", correct);
    println!("Hey, guess a number 1-10.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. Error: {e}.");
                continue;
            }
        };

        // let mut message = if correct > guess {
        //     String::from("That's too low.")
        // } else if correct < guess {
        //     String::from("That's too high.")
        // } else {
        //     String::from(" That's correct.")
        // }

        match guess.cmp(&correct) {
            Ordering::Less => {
                println!("That's too low.");
            }
            Ordering::Greater => {
                println!("That's too high.");
            }
            Ordering::Equal => {
                println!("That's correct.");
                break;
            }
        };
    }
}
