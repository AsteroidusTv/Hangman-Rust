use std::io;
use std::io::prelude::*;

fn main() {
    let word = ask_word();
    let mut counter: u32 = 0;
    let mut right = Vec::new();
    let mut dont = Vec::new();

    loop {
        let letter = ask_char();
        let count = count_occurrences(letter, &word);
        counter += 1;

        if word.chars().any(|c| c.to_ascii_lowercase() == letter.to_ascii_lowercase()) {
            if dont.contains(&letter) {
                println!("This letter has already been entered");
                break;
            } else {
                println!("Yes, the word contains the letter {}", letter);
                println!("{}", counter);
                for _ in 0..count {
                    add_char(&mut right, letter);
                }
            }
        } else {
            if dont.contains(&letter) {
                println!("This letter has already been entered");
                break;
            } else {
                println!("{}", counter);
                add_char(&mut dont, letter);
                println!("No, {:?} does not contain the letter {}", word, letter);
            }
        }

        sort_vector_by_string(&word, &mut right);
        println!("{:?}", right);
        println!("False letters: {:?}", dont);

        if verify_word(&right, &word) {
            println!("You win!");
            break;
        }

        if counter == 10 {
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

fn sort_vector_by_string(input_string: &str, input_vector: &mut Vec<char>) {
    let mut sorted_chars = Vec::new();

    for c in input_string.chars() {
        if let Some(index) = input_vector.iter().position(|&x| x == c) {
            sorted_chars.push(input_vector[index]);
            input_vector.remove(index);
        } else {
            sorted_chars.push('_');
        }
    }

    sorted_chars.extend_from_slice(input_vector);
    *input_vector = sorted_chars;
}
