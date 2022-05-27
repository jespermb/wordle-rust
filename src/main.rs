use colored::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;
// use std::iter::Peekable;

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

#[derive(PartialEq, Debug)]
enum CharState {
    Correct,
    Wrong,
    Exists,
}
#[derive(Debug)]
struct CharacterMap {
    character: char,
    value: CharState,
}

fn check_word_correct(game_word: &str, chosen_word: &str) -> Vec<CharacterMap> {
    let game_word_count = count_unique_characters(&game_word);
    let mut state: Vec<CharacterMap> = Vec::new();
    let mut current_word_count: HashMap<String, usize> = HashMap::new();

    for (i, c) in chosen_word.chars().enumerate() {
        let mut map = CharacterMap {
            character: c,
            value: CharState::Wrong,
        };
        if does_character_exist(c, game_word) && is_position_correct(c, i, game_word) {
            map.value = CharState::Correct;
            if current_word_count.contains_key(&String::from(c)) {
                current_word_count
                    .insert(String::from(c), current_word_count[&String::from(c)] + 1);
            } else {
                current_word_count.insert(String::from(c), 1 as usize);
            }
        }
        state.push(map);
    }

    for (i, c) in chosen_word.chars().enumerate() {
        let char = &String::from(c) as &str;
        if does_character_exist(c, game_word) {
            let chosen_char = game_word_count.get(&String::from(c)).unwrap();

            let mut current_char_count: &usize = &0;
            if current_word_count.contains_key(char) {
                current_char_count = current_word_count.get(char).unwrap();
            }
            let mut map = state.get_mut(i).unwrap();
            if !is_position_correct(c, i, game_word) && current_char_count < chosen_char {
                map.value = CharState::Exists;
                if current_word_count.contains_key(&String::from(c)) {
                    current_word_count
                        .insert(String::from(c), current_word_count[&String::from(c)] + 1);
                } else {
                    current_word_count.insert(String::from(c), 1 as usize);
                }
            }
        }
    }
    return state;
}

fn does_character_exist(char: char, word: &str) -> bool {
    return !word.find(char).is_none();
}

fn is_position_correct(char: char, index: usize, word: &str) -> bool {
    return char == word.chars().nth(index).unwrap();
}

fn count_unique_characters(word: &str) -> HashMap<String, usize> {
    let mut char_count: HashMap<String, usize> = HashMap::new();
    for char in word.chars() {
        if char_count.contains_key(&String::from(char)) {
            char_count.insert(String::from(char), char_count[&String::from(char)] + 1);
        } else {
            char_count.insert(String::from(char), 1);
        }
    }

    return char_count;
}

// struct SequentialCount<I>
// where
//     I: Iterator,
// {
//     iter: Peekable<I>,
// }
// impl<I> SequentialCount<I>
// where
//     I: Iterator,
// {
//     fn new(iter: I) -> Self {
//         SequentialCount {
//             iter: iter.peekable(),
//         }
//     }
// }
// impl<I> Iterator for SequentialCount<I>
// where
//     I: Iterator,
//     I::Item: Eq,
// {
//     type Item = (I::Item, usize);

//     fn next(&mut self) -> Option<Self::Item> {
//         // Check the next value in the inner iterator
//         match self.iter.next() {
//             // There is a value, so keep it
//             Some(head) => {
//                 // We've seen one value so far
//                 let mut count = 1;
//                 // Check to see what the next value is without
//                 // actually advancing the inner iterator
//                 while self.iter.peek() == Some(&head) {
//                     // It's the same value, so go ahead and consume it
//                     self.iter.next();
//                     count += 1;
//                 }
//                 // The next element doesn't match the current value
//                 // complete this iteration
//                 Some((head, count))
//             }
//             // The inner iterator is complete, so we are also complete
//             None => None,
//         }
//     }
// }

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
                let character = String::from(char.character);
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

    #[test]
    fn test_check_word_correct_correct() {
        let correct = check_word_correct("apple", "apple");
        assert_eq!(correct[0].value, CharState::Correct);
        assert_eq!(correct[1].value, CharState::Correct);
        assert_eq!(correct[2].value, CharState::Correct);
        assert_eq!(correct[3].value, CharState::Correct);
        assert_eq!(correct[4].value, CharState::Correct);
    }

    #[test]
    fn test_check_word_correct_wrong() {
        let correct = check_word_correct("brown", "apple");
        assert_eq!(correct[0].value, CharState::Wrong);
        assert_eq!(correct[1].value, CharState::Wrong);
        assert_eq!(correct[2].value, CharState::Wrong);
        assert_eq!(correct[3].value, CharState::Wrong);
        assert_eq!(correct[4].value, CharState::Wrong);
    }

    #[test]
    fn test_check_word_correct_mixed() {
        let correct = check_word_correct("cheer", "close");
        assert_eq!(correct[0].value, CharState::Correct);
        assert_eq!(correct[1].value, CharState::Wrong);
        assert_eq!(correct[2].value, CharState::Wrong);
        assert_eq!(correct[3].value, CharState::Wrong);
        assert_eq!(correct[4].value, CharState::Exists);
    }

    #[test]
    fn test_check_word_correct_mixed_double_e() {
        let correct = check_word_correct("close", "elder");
        println!("{:?}", correct);
        assert_eq!(correct[0].value, CharState::Exists);
        assert_eq!(correct[1].value, CharState::Correct);
        assert_eq!(correct[2].value, CharState::Wrong);
        assert_eq!(correct[3].value, CharState::Wrong);
        assert_eq!(correct[4].value, CharState::Wrong);
    }
}
