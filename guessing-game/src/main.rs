use std::{ io, cmp::Ordering};
use rand::{self, Rng};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1 ..= 100);
    println!("The secret number is : {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = guess.trim().parse()
            .expect("Please use a number - Leo Rawr");

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Too Big"),
            Ordering::Less  => println!("Too Small")
        }
    }
}
