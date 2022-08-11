use rand::seq::SliceRandom;

fn main() {
    let user_word_list = vec!["news", "paper", "elephant", "music"];
    let attacker_word_list = vec!["news", "paper", "elephant", "music"];
    println!(
        "Using exact word list: Over 1M cracks, mean number of guesses was {:?}",
        get_mean_number_of_guesses(&user_word_list, &attacker_word_list)
    );
    let attacker_word_list = vec!["news", "paper", "elephant", "music", "newspaper"];
    println!(
        "Using plus-one word list: Over 1M cracks, mean number of guesses was {:?}",
        get_mean_number_of_guesses(&user_word_list, &attacker_word_list)
    );
}

fn get_mean_number_of_guesses(user_word_list: &[&str], attacker_word_list: &[&str]) -> f64 {
    let mut number_of_guesses_required = vec![];
    for _p in 0..1000000 {
        let user_password = make_a_password(user_word_list);
        let mut attacker_guess = "".to_string();
        let mut number_of_guesses: usize = 0;
        while user_password != attacker_guess {
            attacker_guess = make_a_password(attacker_word_list);
            number_of_guesses += 1;
        }
        // println!("Cracked! Took {:?} guesses this time.", number_of_guesses);
        number_of_guesses_required.push(number_of_guesses);
    }
    mean_number_of_guesses(number_of_guesses_required)
}

fn make_a_password(list: &[&str]) -> String {
    let mut password = "".to_string();
    for _n in 0..3 {
        match list.choose(&mut rand::thread_rng()) {
            Some(word) => {
                password = password + word;
            }
            None => (),
        }
    }
    password
}

fn mean_number_of_guesses(guesses_vec: Vec<usize>) -> f64 {
    let sum: usize = guesses_vec.iter().sum();
    let number_of_guesses = guesses_vec.len();
    sum as f64 / number_of_guesses as f64
}
