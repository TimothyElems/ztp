/// Prerequisites for the lessons
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Rust is not Golang. You can have multiple main functions in a dir
pub fn guess() {
    let secret_number = rand::rng().random_range(1..=17);
    println!("Think of a number between 1 and 17");

    loop {
        println!("Please input your guess");

        // A variable will need to take the value of the guess... which means it'll have to be mutable
        let mut guess = String::new();

        // This is a function call that'll read the input from the cli
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        // So far we have a var for the number, var for the guess, and logic to read the guess.
        // Now we need conditionals to evaluate it all and act appropriately
        println!("You guessed: {}", guess);

        // Add logic so it checks if it's high, low, or just right!
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low\n"),
            Ordering::Greater => println!("Too high\n"),
            Ordering::Equal => {
                println!("Heck yeah!!!");
                break;
            }
        }
    }
}
