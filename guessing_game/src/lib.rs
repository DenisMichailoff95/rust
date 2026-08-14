use std::cmp::Ordering;

pub fn run_game_with_secret(secret_number: u32, input: &str) -> String {
    let mut out = String::new();

    out.push_str("Guess the number!\n");
    out.push_str(&format!("The secret number is: {}\n", secret_number));

    for line in input.lines() {
        out.push_str("Please input your guess.\n");

        let guess_trim = line.trim();
        let guess: u32 = match guess_trim.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        out.push_str(&format!("You guessed: {}\n", guess));

        match guess.cmp(&secret_number) {
            Ordering::Less => out.push_str("Too small!\n"),
            Ordering::Greater => out.push_str("Too big!\n"),
            Ordering::Equal => {
                out.push_str("You win!\n");
                break;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::run_game_with_secret;

    #[test]
    fn wins_after_correct_guess_sequence() {
        let input = "10\n90\n42\n";
        let output = run_game_with_secret(42, input);

        assert!(output.contains("Guess the number!"));
        assert!(output.contains("The secret number is: 42"));
        assert!(output.contains("Too small!"));
        assert!(output.contains("Too big!"));
        assert!(output.contains("You win!"));
    }

    #[test]
    fn ignores_non_numeric_input() {
        let input = "foo\n-\n100\n";
        let output = run_game_with_secret(100, input);

        assert!(output.contains("You win!"));
    }
}
