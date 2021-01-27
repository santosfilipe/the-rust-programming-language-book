use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // gen_range() was updated in newer rand versions and the syntax (a..b) is used instead of (a, b).
    // There is an issue already opened for this on the book repository.
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    // Although in the book this line is removed, I will maintain it.
    println!("The secret number is {}.", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
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
