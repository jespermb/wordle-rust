use rand::seq::SliceRandom;

fn choose_word() -> String {
    let words: Vec<&str> = vec!["apple", "green", "brown", "elder"];
    let chosen_word: Option<&&str> = words.choose(&mut rand::thread_rng());
    return chosen_word.unwrap().to_string();
}

fn main() {
    let word = choose_word();
    println!("Chosen word: {}", word);
}
