use rand::seq::SliceRandom;

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
    println!("Chosen word: {}", word);
}
