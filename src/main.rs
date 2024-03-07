use std::io;

fn main() {
    println!("Guess the number:");

    println!("Write a number: ");
    let mut guess: String = String::new();

     io::stdin()
         .read_line(&mut guess)
         .expect("Can't read your number");

    println!("Você chutou: {guess}");
}
