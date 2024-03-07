mod game;
use rand::Rng;
use game::game_start;

fn main() {
    println!("Guess the number:");

    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..=100);

    game_start(secret_number);
}
