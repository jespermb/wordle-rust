use colored::*;
mod game;
// use std::iter::Peekable;

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
    let word = game::choose_word();

    let mut input: String;
    let mut number_of_guesses = 0;
    println!("Please enter your first word");
    loop {
        input = game::read_one().trim().to_lowercase();
        number_of_guesses += 1;

        if input.chars().count() == WORD_LENGTH {
            let correct = game::check_word_correct(&word, &input);
            let mut is_correct = true;
            for char in correct {
                let character = String::from(char.character);
                if char.value == game::CharState::Correct {
                    print!("{}", character.green());
                }
                if char.value == game::CharState::Exists {
                    is_correct = false;
                    print!("{}", character.yellow());
                }
                if char.value == game::CharState::Wrong {
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
