use std::io;
use std::io::prelude::*;

fn main() {
    let word = ask_word();
    let mut counter: u32 = 0;
    let mut right = Vec::new();
    let mut dont = Vec::new();
    }

    loop {
        let letter = ask_char();
        counter += 1;

        if word.chars().any(|c| c.to_ascii_lowercase() == letter.to_ascii_lowercase()) {
                println!("Yes, the word contains the letter {}", letter);
                println!("{}", counter);
                add_char(&mut right, letter);
        } 
        else {
            loop {
                if dont.contains(&letter) {
                    println!("This letter has already been entered");
                    break
                }

                else {
                    println!("{}", counter);
                    add_char(&mut dont, letter);
                    println!("No, {:?} does not contain the letter {}", word, letter);
                    break;
                }
            }
        }

        verify_word(&mut right, &word);
        if verify_word(&mut right, &word) {
            println!("YOU WIN !");
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

fn verify_word(vec: &mut Vec<char>, word: &str) -> bool {
    let vector_char: Vec<char> = vec.iter().cloned().collect();
    let word_chars: Vec<char> = word.chars().collect();

    if vector_char.len() == word_chars.len() {
        vector_char == word_chars
    } else {
        false
    }
}

// Experimental
// fn count_letter_occurrences(word: &str) -> Vec<(char, i32)> {
//     let mut occurrences = Vec::new();
//     let mut counts = std::collections::HashMap::new();

//     for letter in word.chars() {
//         *counts.entry(letter).or_insert(0) += 1;
//     }

//     for (letter, count) in counts {
//         if count > 1 {
//             occurrences.push((letter, count));
//         }
//     }

//     occurrences
// }
