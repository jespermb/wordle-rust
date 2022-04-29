use colored::*;
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
    let mut number_of_guesses = 0;
    println!("Please enter your first word");
    loop {
        input = read_one().trim().to_lowercase();
        number_of_guesses += 1;

        if input.chars().count() == WORD_LENGTH {
            let correct = check_word_correct(&word, &input);
            let mut is_correct = true;
            for char in correct {
                let character = String::from(char.character.to_string());
                if char.value == CharState::Correct {
                    print!("{}", character.green());
                }
                if char.value == CharState::Exists {
                    is_correct = false;
                    print!("{}", character.yellow());
                }
                if char.value == CharState::Wrong {
                    is_correct = false;
                    print!("{}", character.truecolor(109, 109, 109));
                }
            }
            println!("");
            if is_correct {
                println!("You win!");
                break;
            }
            if number_of_guesses == 6 {
                println!("You lose!");
                break;
            }
        } else {
            println!("Invalid word. Please enter a word thats 5 characters long.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_word() {
        let word = choose_word();
        assert_eq!(word.chars().count(), 5);
    }

    #[test]
    fn test_does_character_exist() {
        let word = "apple";
        assert_eq!(does_character_exist('a', word), true);
        assert_eq!(does_character_exist('b', word), false);
    }

    #[test]
    fn test_is_position_correct() {
        let word = "apple";
        assert_eq!(is_position_correct('l', 3, word), true);
        assert_eq!(is_position_correct('p', 0, word), false);
    }
}
