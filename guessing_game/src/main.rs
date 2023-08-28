use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // echos to console
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("The secret number is: {secret_number}");

     // echos to console
    loop {
        println!("Please input your guess.");

        // mutable variable guess, type string that is new or empty
        let mut guess = String::new();

        // Three lines of code but act as a single line because of the semi-colon ; tab for organization only
        io::stdin()
            .read_line(&mut guess) // put the input into the mutable variable guess and use a reference to allocate memory accordingly??
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
