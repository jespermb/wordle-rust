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

fn main() {
    let word = choose_word();
    println!("Please enter your first word");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    println!("Words: {} / {}", word, input);
}
