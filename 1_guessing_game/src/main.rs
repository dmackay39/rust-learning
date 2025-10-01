use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    // thread_rng gives a random number local to the thread of execution and seeded by the OS.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut entails that the variable is mutable.

        io::stdin()
            .read_line(&mut guess) //& indicates passing a reference. Need mut to make the reference mutable.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //the new guess variable shadows the previous one.

        println!("You guessed: {guess}"); //curly braces for placeholder.

        match guess.cmp(&secret_number) { //cmo method compares numbers and returns an Ordering enum.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}