use std::io;
use std::io::prelude::*;

fn main() {
    let word = "hello";
    let mut counter: u32 = 0;
    let mut right = Vec::new();
    let mut dont = Vec::new();

    loop {
        let letter = ask_char();
        counter += 1;

        if word.chars().any(|c| c.to_ascii_lowercase() == letter.to_ascii_lowercase()) {
            println!("Oui, {:?} contient la lettre {}", word, letter);
            println!("{}", counter);
            add_char(&mut right, letter);
        } else {
            println!("Non, {:?} ne contient pas la lettre {}", word, letter);
            println!("{}", counter);
            add_char(&mut dont, letter);
            if counter == 10 {
                println!("Tu as perdu !")
            }
        }
    }
}

fn ask_char() -> char {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Échec de la lecture de la ligne");
    line.chars()
        .next()
        .expect("Aucun caractère saisi")
        .to_ascii_lowercase()
}

fn add_char(vec: &mut Vec<char>, letter: char) {
    vec.push(letter);
    println!("{:?}", vec);
}

fn verify_word(vec: &mut Vec<char>, word: &str) -> bool {
   let vector_char: Vec<char> = vec.iter().cloned().collect();
    let word_chars: Vec<char> = word.chars().collect();

    if vector_char.len() != word_chars.len() {
        return false;
    }

    vector_char == word_chars
}

