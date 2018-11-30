extern crate rand;

use std::io; // guess what this is.
/* if we haven't used use std::io we could also
   call std::io::stdin */
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");
        let mut guess = String::new(); // in rust, variables are immutable by default;

        io::stdin()
            .read_line(&mut guess) //  The & indicates that this argument is a reference
            .expect("Failed to read line"); // read_line returns io::Result, which can either be an Ok or Err value.
        /* If the previous io::Result value
           is an Ok, expect simply returns the value,
           if the result is an Err, expect 
           throws an Error with the message passed in
           as parameter 
           
           Not calling expect throws a warning
           at compile time
           */

        // rust allows us to shadow the previous value of guess
        // with a new one - when referencing a string, guess is the
        // first value, when referencing u32, guess is the second.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess); // the parameter will replace the curly brackets. Future curlies will replace further parameters.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
