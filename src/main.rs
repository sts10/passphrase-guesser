use rand::seq::SliceRandom;

fn main() {
    let user_word_list = vec!["news", "paper", "elephant", "music"];
    let superset_attacker_word_list = vec!["news", "paper", "elephant", "music", "newspaper"];
    println!(
        "Using user's exact word list and guessing randomly: Over 1M cracks, mean number of guesses was {:.4}",
        random_guessing_attack(&user_word_list, &user_word_list)
    );
    println!(
        "Using a super-set word list and guessing randomly: Over 1M cracks, mean number of guesses was {:.4}",
        random_guessing_attack(&user_word_list, &superset_attacker_word_list)
    );
    println!("---------------------------------");
    println!(
        "Using user's word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was {:.4}",
        three_word_brute_force_attack(&user_word_list, &user_word_list)
    );
    println!(
        "Using a super-set word list and a three-word brute-force procedure: Over 1M cracks, mean number of guesses was {:.4}",
        three_word_brute_force_attack(&user_word_list, &superset_attacker_word_list)
    );
    println!("---------------------------------");
    let (mean_number_of_guesses, percentage_of_cracks_early) =
        gradual_word_brute_force_attack(&user_word_list, &user_word_list);
    println!(
        "Using user's word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was {:.4}. {:.4}% of cracks took fewer than 25 guesses.",
        mean_number_of_guesses, percentage_of_cracks_early
    );
    let (mean_number_of_guesses, percentage_of_cracks_early) =
        gradual_word_brute_force_attack(&user_word_list, &superset_attacker_word_list);
    println!(
        "Using a super-set word list and a two-word-then-three-word brute-force procedure: Over 1M cracks, mean number of guesses was {:.4}. {:.4}% of cracks took fewer than 25 guesses.",
        mean_number_of_guesses, percentage_of_cracks_early
    );
    println!("---------------------------------");
    let (mean_number_of_guesses, percentage_of_cracks_early) = unlucky_user(&user_word_list);
    println!(
        "Using user's word list and a two-word-then-three-word brute-force procedure against a very unlucky user: Over 1M cracks, mean number of guesses was {:.4}. {:.4}% of cracks took fewer than 25 guesses.",
        mean_number_of_guesses, percentage_of_cracks_early
    );
    let (mean_number_of_guesses, percentage_of_cracks_early) =
        unlucky_user(&superset_attacker_word_list);
    println!(
        "Using a super-set word list and a two-word-then-three-word brute-force procedure against a very unlucky user: Over 1M cracks, mean number of guesses was {:.4}. {:.4}% of cracks took fewer than 25 guesses.",
        mean_number_of_guesses, percentage_of_cracks_early
    );
}

fn random_guessing_attack(user_word_list: &[&str], attacker_word_list: &[&str]) -> f64 {
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

fn three_word_brute_force_attack(user_word_list: &[&str], attacker_word_list: &[&str]) -> f64 {
    let mut number_of_guesses_required = vec![];
    for _p in 0..1000000 {
        let user_password = make_a_password(user_word_list);
        let mut attacker_guess = "".to_string();
        let mut number_of_guesses: usize = 0;
        'outer: while user_password != attacker_guess {
            for word1 in attacker_word_list {
                for word2 in attacker_word_list {
                    for word3 in attacker_word_list {
                        attacker_guess =
                            format!("{:?}{:?}{:?}", word1, word2, word3).replace("\"", "");
                        number_of_guesses += 1;
                        if attacker_guess == user_password {
                            break 'outer;
                        }
                    }
                }
            }
        }
        // println!("Cracked! Took {:?} guesses this time.", number_of_guesses);
        number_of_guesses_required.push(number_of_guesses);
    }
    mean_number_of_guesses(number_of_guesses_required)
}

fn gradual_word_brute_force_attack(
    user_word_list: &[&str],
    attacker_word_list: &[&str],
) -> (f64, f64) {
    let mut number_of_guesses_required = vec![];
    for _p in 0..1000000 {
        let user_password = make_a_password(user_word_list);
        let mut attacker_guess = "".to_string();
        let mut number_of_guesses: usize = 0;
        'outer: while user_password != attacker_guess {
            // Two words
            for word1 in attacker_word_list {
                for word2 in attacker_word_list {
                    attacker_guess = format!("{:?}{:?}", word1, word2).replace("\"", "");
                    number_of_guesses += 1;
                    if attacker_guess == user_password {
                        break 'outer;
                    }
                }
            }
            // Three words
            for word1 in attacker_word_list {
                for word2 in attacker_word_list {
                    for word3 in attacker_word_list {
                        attacker_guess =
                            format!("{:?}{:?}{:?}", word1, word2, word3).replace("\"", "");
                        number_of_guesses += 1;
                        if attacker_guess == user_password {
                            break 'outer;
                        }
                    }
                }
            }
        }
        // println!("Cracked! Took {:?} guesses this time.", number_of_guesses);
        number_of_guesses_required.push(number_of_guesses);
    }
    let mut number_of_early_cracks = 0;
    for guesses in &number_of_guesses_required {
        // For now, we're going to arbritarily define an "early" crack as < 25 guesses
        if guesses < &25 {
            number_of_early_cracks += 1;
        }
    }
    let percentage_of_cracks_the_were_early: f64 =
        number_of_early_cracks as f64 / 1000000.0 * 100.0;

    (
        mean_number_of_guesses(number_of_guesses_required),
        percentage_of_cracks_the_were_early,
    )
}

fn unlucky_user(attacker_word_list: &[&str]) -> (f64, f64) {
    let mut number_of_guesses_required = vec![];
    for _p in 0..1000000 {
        let user_password = match vec![
            "newspaperelephant",
            "newspapermusic",
            "elephantnewspaper",
            "musicnewspaper",
            "newspapernews",
            "newspaperpaper",
            "newsnewspaper",
            "papernewspaper",
        ]
        .choose(&mut rand::thread_rng())
        {
            Some(unlucky_passphrase) => unlucky_passphrase.to_string(),
            None => panic!("Unable to pick an unlucky passphrase"),
        };
        let mut attacker_guess = "".to_string();
        let mut number_of_guesses: usize = 0;
        'outer: while user_password != attacker_guess {
            // Two words
            for word1 in attacker_word_list {
                for word2 in attacker_word_list {
                    attacker_guess = format!("{:?}{:?}", word1, word2).replace("\"", "");
                    number_of_guesses += 1;
                    if attacker_guess == user_password {
                        break 'outer;
                    }
                }
            }
            // Three words
            for word1 in attacker_word_list {
                for word2 in attacker_word_list {
                    for word3 in attacker_word_list {
                        attacker_guess =
                            format!("{:?}{:?}{:?}", word1, word2, word3).replace("\"", "");
                        number_of_guesses += 1;
                        if attacker_guess == user_password {
                            break 'outer;
                        }
                    }
                }
            }
        }
        // println!("Cracked! Took {:?} guesses this time.", number_of_guesses);
        number_of_guesses_required.push(number_of_guesses);
    }
    let mut number_of_early_cracks = 0;
    for guesses in &number_of_guesses_required {
        // For now, we're going to arbritarily define an "early" crack as < 25 guesses
        if guesses < &25 {
            number_of_early_cracks += 1;
        }
    }
    let percentage_of_cracks_the_were_early: f64 =
        number_of_early_cracks as f64 / 1000000.0 * 100.0;

    (
        mean_number_of_guesses(number_of_guesses_required),
        percentage_of_cracks_the_were_early,
    )
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
