use rand::seq::SliceRandom;
use std::io;

/// Generate a random word
///
/// # Examples
///
/// ```
/// let word = choose_word();
/// ```
fn choose_word() -> String {
    let words: Vec<&str> = vec!["apple", "green", "brown", "elder"];
    let chosen_word: Option<&&str> = words.choose(&mut rand::thread_rng());
    return chosen_word.unwrap().to_string();
}
#[test]
fn test_choose_word() {
    let word = choose_word();
    assert_eq!(word.chars().count(), 5);
}

fn read_one() -> String {
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .ok()
        .expect("Please enter a word");
    word
}

fn main() {
    const WORD_LENGTH: usize = 5;
    let word = choose_word();

    let mut input: String;
    loop {
        println!("Please enter your first word");
        input = read_one().trim().to_lowercase();

        if input.chars().count() == WORD_LENGTH {
            break;
        }

        println!("Invalid word. Please enter a word thats 5 characters long.")
    }

    println!("Words: {} / {}", word, input);
}
