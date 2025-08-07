// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Game Start
    println!("Welcome to CS110L Hangman!");

    let mut counter = 0;
    let mut correct_guess_word_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guesses = String::new();

    loop {
        let correct_guess_word: String = correct_guess_word_chars.iter().collect();
        println!("The word so far is {}", correct_guess_word);
        println!("You have guessed the following letters: {}", guesses);
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - counter);
        print!("Please guess a letter: ");
        io::stdout().flush().unwrap(); // Forces the output to appear

        let mut guess = String::new();
        let _ = io::stdin().read_line(&mut guess);
        let guess_letter = guess.chars().nth(0).expect("");
        let mut is_guess_correct = false;

        guesses.push_str(&guess_letter.to_string());


        for (i, c) in secret_word_chars.iter().enumerate() {
            if correct_guess_word_chars[i] == '-' && secret_word_chars[i] == guess_letter {
                correct_guess_word_chars[i] = c.clone();
                is_guess_correct = true;
                println!("");
                break;
            }
        }
        
        if !is_guess_correct {
            counter += 1;
            println!("Sorry, that letter is not in the word\n");
        }

        if !correct_guess_word_chars.contains(&'-') { break; }

        if counter >= NUM_INCORRECT_GUESSES { break; }
    }

    // Game End
    if counter >= NUM_INCORRECT_GUESSES {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    }
}
