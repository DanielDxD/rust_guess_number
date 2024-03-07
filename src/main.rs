use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number:");

    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);

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
