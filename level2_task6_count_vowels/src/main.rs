fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| {
            matches!(
                *c,
                'a' | 'e'
                    | 'i'
                    | 'o'
                    | 'u'
                    | 'y'
                    | 'A'
                    | 'E'
                    | 'I'
                    | 'O'
                    | 'U'
                    | 'Y'
                    | 'а'
                    | 'е'
                    | 'ё'
                    | 'и'
                    | 'о'
                    | 'у'
                    | 'ы'
                    | 'э'
                    | 'ю'
                    | 'я'
                    | 'А'
                    | 'Е'
                    | 'Ё'
                    | 'И'
                    | 'О'
                    | 'У'
                    | 'Ы'
                    | 'Э'
                    | 'Ю'
                    | 'Я'
            )
        })
        .count()
}

fn main() {
    let text = "Привет, world! Аеёиоу";
    println!("Vowels in \"{text}\": {}", count_vowels(text));
}
