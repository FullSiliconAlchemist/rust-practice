use std::io;
use rand::{prelude::thread_rng, Rng};
use rand::random;

use crate::lesson::Lesson;

pub struct Lesson5 { pub name: String }

impl Lesson for Lesson5 {
    fn run(&self) -> () {
        println!("\n{}:", self.name);

        io_example();
        let number = random::<i32>();
        println!("Random number is {}", number);

        println!("\n\nCHALLENGE: RANDOM NUMBER GEN!");
        random_nm_game();
    }
}

fn random_nm_game() -> () {
    let solution = thread_rng().gen_range(1..101);
    let mut buffer = String::new();
    let mut guess = -1;

    println!("Guess a number between 1 and 100:");
    while guess != solution {
        if guess != -1 {
            match guess {
                a if a > solution => println!("{} Is not correct... Lower!", a),
                b if b < solution => println!("{} Is not correct... Higher!", b),
                _ => ()
            }
        }

        let _ = io::stdin().read_line(&mut buffer).expect("Failed to read the line...");
        guess = buffer.trim().parse::<i32>().expect("Failed to parse the number. Numbers only!");
        buffer.clear();
    }

    println!("You guessed it! {}", solution);
}

fn io_example() -> () {
    println!("\n\nModules:");
    let mut buffer = String::new();
    println!("Enter a number:");
    let _ = io::stdin().read_line(&mut buffer);
    println!("Buffer result is: {}", buffer);
    let number = buffer.trim().parse::<i32>().unwrap_or_default(); // chooses default of datatype
    println!("The number result is: {}", number + 1);
}
