use std::io;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn beats(a: Move, b: Move) -> bool {
    matches!((a, b),
        (Move::Rock, Move::Scissors) |
        (Move::Paper, Move::Rock) |
        (Move::Scissors, Move::Paper)
    )
}

fn main() {
    let mut rng = rand::thread_rng();
    let choices = [Move::Rock, Move::Paper, Move::Scissors];
    loop {
        println!("Enter rock/paper/scissors or quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        let player = match input.as_str() {
            "rock" => Move::Rock,
            "paper" => Move::Paper,
            "scissors" => Move::Scissors,
            "quit" => break,
            _ => {
                println!("Invalid");
                continue;
            }
        };
        let computer = choices[rng.gen_range(0..3)];
        println!("You: {:?}, Computer: {:?}", player, computer);
        if player == computer {
            println!("Draw!");
        } else if beats(player, computer) {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }
}
