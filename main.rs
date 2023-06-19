use std::io;
use std::io::prelude::*;

fn main() {
   let word = "Hello";
   let mut letter = ask_char();
   let mut counter = 1;
   let mut right = vec!['a'];
   let mut dont = vec!['b'];
   
   loop {   
      counter += 1;
      if word.chars().any(|c| c.to_ascii_lowercase() == letter.to_ascii_lowercase()) {
         println!("Yes, {} contains the letter {}", word, letter);
         letter = ask_char();
         println!("{}", counter);
         add_char(&mut right, letter)
         
         }
      else if word.chars().any(|c| c.to_ascii_lowercase() != letter.to_ascii_lowercase()) {
         println!("No, {} doesn't contains the letter {}", word, letter);
         letter = ask_char();
         println!("{}", counter);
         add_char(&mut dont, letter)

      };
   }


}

fn ask_char() -> char {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).expect("Failed to read line");
    line.chars().next().expect("No character entered").to_ascii_lowercase()
}

fn add_char(vec: &mut Vec<char>, letter: char) {
      vec.push(letter);
      println!("{:?}", vec)
}