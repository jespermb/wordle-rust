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

#[derive(PartialEq)]
enum CharState {
    Correct,
    Wrong,
    Exists,
}
struct CharacterMap {
    character: char,
    value: CharState,
}

fn check_word_correct(word: &str, chosen_word: &str) -> Vec<CharacterMap> {
    let mut state: Vec<CharacterMap> = Vec::new();
    for (i, c) in chosen_word.chars().enumerate() {
        let mut map = CharacterMap {
            character: c,
            value: CharState::Wrong,
        };
        if does_character_exist(c, word) {
            map.value = CharState::Exists;
            if is_position_correct(c, i, word) {
                map.value = CharState::Correct;
            }
        }
        state.push(map);
    }
    return state;
}

fn does_character_exist(char: char, word: &str) -> bool {
    return !word.find(char).is_none();
}

fn is_position_correct(char: char, index: usize, word: &str) -> bool {
    return char == word.chars().nth(index).unwrap();
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

    let correct = check_word_correct(&word, &input);
    for char in correct {
        if char.value == CharState::Correct {
            println!("Character {} is correct", char.character);
        }
        if char.value == CharState::Exists {
            println!("Character {} exists", char.character);
        }
        if char.value == CharState::Wrong {
            println!("Character {} not found", char.character);
        }
    }
    println!("Words: {} / {}", word, input);
}
