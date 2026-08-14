use rand::Rng;
use std::io::{self, Read};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let output = guessing_game::run_game_with_secret(secret_number, &input);
    print!("{}", output);
}
