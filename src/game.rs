use std::cmp::Ordering;
use std::io;

pub fn game_start(secret_number: u32) {
    loop {
        println!("Write a number: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Can't read your number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("The secret number is smaller"),
            Ordering::Less => println!("The secret number is bigger")
        }
    }
}