// standard library, io library
use std::io;
// ordering lib
use std::cmp::Ordering;
// random library, Rng trait
use rand::Rng;


fn main() {

    // macro to print msg
    println!("Guess the number!");

    // vars immutable by default, get rand num
    let secret_number = rand::thread_rng().gen_range(1..101);

    // loop (infinite)
    loop {
        println!("Please input your guess.");

        // mutable var, new string
        let mut guess = String::new();

        // read_line() returns io::Result, if error print err, else get input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // can reuse var name - "shadows" previous var
        // trim elims whitespace, parse to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match statement - made up of 'arms', consisting of a pattern
        // and code to run if it matches, looks through patterns
        // sequentially
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
