use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game.");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("the secret number is {}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please enter your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("this is not a number !!");
                continue;
            }
        };

        println!("your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
