use std::cmp::Ordering;
use std::io; // bring io library into scope
use rand::Rng; // bring Rng trait into scope

fn main() {
    println!("Guess the number!");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { // infinite loop
        println!("Please input your guess.");

        // In Rust, variables are immutable by default.
        // To make them mutable, use the mut keyword.
        let mut guess = String::new();

        // read_line returns a Result value.
        // Result types are enumerations, which have variants.
        // The Ok variant indicates the operation was successful.
        // The Err variant indicates the operation failed.
        // The expect method is called on the Result value.
        // If the Result value is the Err variant, expect will cause the program to crash.
        // If the Result value is the Ok variant, expect will take the return value of the Ok variant and return it to the calling code.
        // The expect method is useful for debugging.
        // If the program crashes, the error message will be displayed.
        io::stdin()
            .read_line(&mut guess)// references are immutable by default so we need to pass mut to make it mutable
            .expect("Failed to read line");

        // The parse method on strings converts a string to another type.
        // Here, we use it to convert from a string to a number.
        // We need to tell Rust the exact number type we want by using let guess: u32.
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type

        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing the previous guess variable

        // another way to handle the error is to use a match expression
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue // ignore errors and continue the loop
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            },
        }
    }
}
