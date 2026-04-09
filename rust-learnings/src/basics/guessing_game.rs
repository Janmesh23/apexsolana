use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ============================================================
// BASICS: Guessing Game Mini-Project
//
// Welcome to the classic Rust Guessing Game!
// In this playground, you'll learn about standard I/O, parsing,
// match statements, and the rand crate.
//
// Fix the code below step-by-step.
// ============================================================

pub fn start() {
    println!("Guess the number!");

    // STEP 1: Generate a random number
    // TODO: Use `rand::thread_rng().gen_range(1..=100)` to generate a secret number.
    let secret_number: u32 = todo!("Generate a random number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // STEP 2: Read user input
        // TODO: Use `io::stdin().read_line()` to properly read the user's input into `guess`.
        // Don't forget to `.expect()` a failure!
        todo!("Read a line from standard input into the `guess` String");

        // STEP 3: Parse and Handle Result
        // TODO: Trim the newline characters off the guess and `.parse()` it into a `u32`.
        // Use a `match` statement. If it's an Ok(num), return it.
        // If it's an Err(_), print an error message and `continue` to the next loop iteration.
        let guess: u32 = match guess.trim().parse() {
            // Fill these arms out!
            _ => todo!("Match Ok() and Err()"),
        };

        println!("You guessed: {guess}");

        // STEP 4: Compare
        // TODO: Use a match statement with `guess.cmp(&secret_number)`.
        // Print messages for `Ordering::Less` and `Ordering::Greater`.
        // If they hit `Ordering::Equal`, print "You win!" and `break` the loop to end the game!
        match guess.cmp(&secret_number) {
            _ => todo!("Match the ordering, and break on Equal"),
        }
    }
}
