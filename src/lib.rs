use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
mod game;

static mut NUMBER_OF_GUESSES: i32 = 0;

fn add_guess() {
    unsafe { NUMBER_OF_GUESSES += 1 };
}

fn get_guess() -> i32 {
    unsafe { NUMBER_OF_GUESSES }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let word = game::choose_word().as_str().to_owned();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let input = document.get_element_by_id("word").unwrap();
    let button = document
        .get_element_by_id("submit")
        .expect("no submit button");
    let game_result = document.get_element_by_id("guess_result").unwrap();
    let word_result = document.get_element_by_id("word_result").unwrap();

    let cb = Closure::<dyn FnMut()>::new(move || {
        let input_element = input.clone();
        let input_value = input_element
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap();
        let input_value = input_value.value();
        let guess_result = guess_result(&word, input_value.as_str());
        game_result.set_text_content(Some(&guess_result));
        let match_result = check_word(&word, input_value.as_str());
        let mut previous_html = word_result.inner_html();
        previous_html.push_str("<br/>");
        previous_html.push_str(&match_result);
        word_result.set_inner_html(&previous_html);
    });
    button
        .dyn_ref::<HtmlElement>()
        .expect("submit button should be an HTML element")
        .set_onclick(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    Ok(())
}

#[wasm_bindgen]
pub fn guess_result(word: &str, name: &str) -> String {
    const WORD_LENGTH: usize = 5;

    add_guess();

    if name.chars().count() == WORD_LENGTH {
        let correct = game::check_word_correct(&word, &name);
        let mut is_correct = true;
        for char in correct {
            if char.value == game::CharState::Exists {
                is_correct = false;
            }
            if char.value == game::CharState::Wrong {
                is_correct = false;
            }
        }
        if is_correct {
            return "You win!".into();
        }
        let guesses = get_guess();
        if guesses >= 6 {
            return "You lose!".into();
        }
        return "Try again!".into();
    }
    return "Invalid word. Please enter a word thats 5 characters long.".into();
}

#[wasm_bindgen]
pub fn check_word(word: &str, name: &str) -> String {
    const WORD_LENGTH: usize = 5;

    if name.chars().count() == WORD_LENGTH {
        let correct = game::check_word_correct(&word, &name);
        let mut results = String::new();
        for char in correct {
            let character = String::from(char.character);
            if char.value == game::CharState::Correct {
                results.push_str(
                    format!(
                        "<span class='character' style='color: green;'>{}</span>",
                        character
                    )
                    .as_str(),
                );
            }
            if char.value == game::CharState::Exists {
                results.push_str(
                    format!(
                        "<span class='character' style='color: yellow;'>{}</span>",
                        character
                    )
                    .as_str(),
                );
            }
            if char.value == game::CharState::Wrong {
                results.push_str(
                    format!(
                        "<span class='character' style='color: grey;'>{}</span>",
                        character
                    )
                    .as_str(),
                );
            }
        }
        return results;
    }
    return "".into();
}
