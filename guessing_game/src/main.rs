use std::io;

// use std::cmp::Ordering;

use rand::Rng;
fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is {secret_number}");

    println!("Input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("You win!")
        // or uncomment the use on line 3, and cut std::cmp::
    }
}
