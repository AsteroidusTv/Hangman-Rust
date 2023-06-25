use std::io;
use std::io::prelude::*;
use std::convert::TryInto;

fn main() {
    let word = ask_word();
    let attempts: u32 = ask_number();
    let mut counter: u32 = word.len().try_into().unwrap();
    let mut right = Vec::new();
    let mut dont = Vec::new();

    loop {
        let letter = ask_char();
        let count = count_occurrences(letter, &word);
        counter += 1;

        if word.chars().any(|c| c.to_ascii_lowercase() == letter.to_ascii_lowercase()) {
            if dont.contains(&letter) {
                println!("This letter has already been entered");
                continue;
            } else {
                println!("Yes, the word contains the letter {}", letter);
                println!("{}", counter);
                for _ in 0..count {
                    add_char(&mut right, letter);
                }
                let result = process_string(&word, right.clone());
                println!("The word : {:?}", result);
            }

        } else {
            if dont.contains(&letter) {
                println!("This letter has already been entered");
                continue;
            } else {
                println!("{}", counter);
                add_char(&mut dont, letter);
                println!("No, {:?} does not contain the letter {}", word, letter);
            }
        }

        if verify_word(&right, &word) {
            println!("You win!, the word was {word}");

            break;
        }

        if counter == attempts {
            println!("You lost!");
            break;
        }
    
    }
}

fn ask_word() -> String {
    println!("Please enter a word to start the game, this word should not be visible to the players: ");
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.trim().to_string()
}

fn ask_char() -> char {
    println!("Please enter a letter");
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.chars()
        .next()
        .expect("No character entered")
        .to_ascii_lowercase()
}

fn ask_number() -> u32 {
    loop {
        let mut line = String::new();
        println!("Enter the number of attempts for the player(s) (by default there is the same number of attempts as the number of letters in the word): ");

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    }
}


fn add_char(vec: &mut Vec<char>, letter: char) {
    vec.push(letter);
    println!("{:?}", vec);
}

fn verify_word(vec: &[char], word: &str) -> bool {
    let mut vector_chars: Vec<char> = vec.iter().cloned().collect();
    let mut word_chars: Vec<char> = word.chars().collect();

    vector_chars.sort();
    word_chars.sort();

    vector_chars == word_chars
}

fn count_occurrences(letter: char, text: &str) -> usize {
    text.chars().filter(|&c| c == letter).count()
}

fn process_string(input: &str, mut vector: Vec<char>) -> Vec<char> {
    vector.sort_by(|a, b| {
        input.find(*a).unwrap_or(input.len()).cmp(&input.find(*b).unwrap_or(input.len()))
    });

    let mut result = vec!['_'; input.len()];

    let mut input_index = 0;
    let mut vector_index = 0;

    while input_index < input.len() {
        if vector_index < vector.len() && vector[vector_index] == input.chars().nth(input_index).unwrap() {
            result[input_index] = vector[vector_index];
            vector_index += 1;
        }

        input_index += 1;
    }

    result
}






