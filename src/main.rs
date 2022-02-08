extern crate colored;
use colored::*;

use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

/// Classify Character Matches for each word
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CharMatch {
    DoesNotExist(char),
    ExistsSomewhereElse(char),
    ExistsHere(char),
}

/// A Single guess attempt
/// TODO consider adding Levenshtien Distance as a score heuristic
#[derive(Debug)]
struct Attempt {
    length: usize,
    attempt_text: String,
    match_vector: Vec<CharMatch>,
    is_win: bool
}

impl Attempt {

    fn new(length: usize) -> Attempt {

        Attempt {
            length,
            attempt_text: String::new(),
            match_vector: Vec::new(),
            is_win: false
        }
    }

    fn attempt(&mut self, attempt_text: &mut String) -> Option<()> {
        if attempt_text.len() != self.length {
            None
        } else {
            self.attempt_text = attempt_text.to_string();
            Some(())
        }
    }

    fn resolve(&mut self, target_string: &String) {

        let mut match_vector: Vec<CharMatch> = Vec::new();
        let mut is_win = 0;

        for (i, c) in self.attempt_text.chars().enumerate() {

            if target_string.contains(c) {

                // check if the position is right
                let index_check = target_string.to_string().as_bytes()[i];

                if (index_check as char) == c {
                    match_vector.push(CharMatch::ExistsHere(c));
                    is_win += 1;
                } else {
                    match_vector.push(CharMatch::ExistsSomewhereElse(c));
                }

            } else {
                match_vector.push(CharMatch::DoesNotExist(c));
            }
        }

        self.match_vector = match_vector;
        self.is_win = if is_win == target_string.len() { true } else { false };
    }

    // TODO figure out how to test this
    fn render(&self) {
        // println!("Entering Match Phase");
        self.match_vector
            .iter()
            .map(|x| {
                match x {
                    &CharMatch::DoesNotExist(char) => {
                        print!("{}{}{}", 
                            " ".to_string().on_black().bright_red(),
                            char.to_string().on_black().bright_red(),
                            " ".to_string().on_black().bright_red(),
                        );
                    }
                    &CharMatch::ExistsSomewhereElse(char) => {
                        print!("{}{}{}", 
                            " ".to_string().on_bright_yellow().black(),
                            char.to_string().on_bright_yellow().black(),
                            " ".to_string().on_bright_yellow().black(),
                        );
                    }
                    &CharMatch::ExistsHere(char) => {
                        print!("{}{}{}", 
                            " ".to_string().on_bright_green().black(),
                            char.to_string().on_bright_green().black(),
                            " ".to_string().on_bright_green().black(),
                        );
                    }
                };
            })
            .last();
        print!("\n");
    }
}


/// Hold all Game Data
struct Game {

    max_attempts: i32,
    attempts: Vec<Attempt>,

    words: HashSet<String>,
    word_length: usize,
    target_word: String,
}

impl Game {

    /// intialize the game, decide the word and load all words
    fn new() -> Game {

        let mut datastore = HashSet::new();

        // get all words
        let words = include_str!("./words.txt").to_string();

        // Load into hashset for ideal O(1) lookup
        words
            .split('\n')
            .map(|word| {
                datastore.insert(word.to_string())
            })
            .last();

        // get Random word
        let mut rng = thread_rng();
        let random_word = datastore.iter().choose(&mut rng).unwrap();

        println!("There are {} words in the dataset", datastore.len());
        println!("The word chosen for this Game is {}", random_word);

        Game {
            max_attempts: 5,
            word_length: 5,
            attempts: Vec::new(),
            target_word: random_word.to_string(),
            words: datastore
        }
    }

    fn make_play(&mut self, mut user_word: String) -> Attempt {

        let mut temp_attempt = Attempt::new(self.word_length);

        // TODO error handling for failed resolve
        temp_attempt.attempt(&mut user_word);
        temp_attempt.resolve(&self.target_word);

        temp_attempt.render();

        temp_attempt
    }

    fn game_loop(&mut self) {

        let mut current_attempt = 1;

        // run the game loop
        while current_attempt <= self.max_attempts + 1 {

            println!("Attempt {}/{}", current_attempt, self.max_attempts + 1);

            // take a guess from the user
            let mut user_word = String::new();
            std::io::stdin().read_line(&mut user_word).unwrap();
            // get rid of the newline
            user_word.pop();

            if user_word.len() != self.word_length {
                println!("Word is not the right length!");
                continue;
            }

            // conver to lowercase
            user_word = user_word.to_lowercase();

            // make sure it exists in the word set
            match self.words.get(&user_word) {
                Some(_) =>  {

                    // if so, run the attempt on it
                    let temp_attempt = self.make_play(user_word);

                    current_attempt += 1;

                    // println!("Win Status {}", temp_attempt.is_win);

                    if temp_attempt.is_win {
                        println!("{}", "Congratulations! You Won :D".on_bright_green().black());
                        break;
                    }

                    self.attempts.push(temp_attempt);
                }
                None => {
                    println!("Not in the word list!");
                    continue;
                }
            }


        }

    }

}

fn main() {
    // let mut words = include_str!("./words.txt");
    //
    //
    //

    let mut a = Game::new();
    a.game_loop();
    /*

    println!("Welcome to CLI-Worlde!");
    println!("Enter your Attempt Below.");

    let word = "FIERY".to_string();
    let mut attempt = String::new();

    for _ in 0..6 {
        print!("> ");
        std::io::stdin().read_line(&mut attempt).unwrap();

        attempt = attempt.to_string().to_uppercase();
        attempt.pop();

        // sanity check
        if attempt.len() != 5 {
            println!("Word Length Wrong! Start Again.");
            break;
        }

        // resolve(&attempt, &word);

        if attempt == word {
            println!("Congratulations!");
            break;
        } else {
            attempt.clear();
        }
    }

    */
}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_attempt() {
        // let target_text = String::from("FIERY");

        let mut test_text_pass = String::from("START");
        let mut test_text_fail_length = String::from("STARTS");

        let mut a = Attempt::new(5);

        assert!(a.attempt(&mut test_text_fail_length).is_none());
        assert!(a.attempt(&mut test_text_pass).is_some());
    }

    #[test]
    fn test_attempt_resolution() {

        let target_text = String::from("FIERY");
        let mut test_text_pass = String::from("FEAST");

        let mut a = Attempt::new(5);

        // will pass
        a.attempt(&mut test_text_pass);

        a.resolve(&target_text);

        // manually match the vectors
        let match_vec = vec![
            CharMatch::ExistsHere('F'),
            CharMatch::ExistsSomewhereElse('E'),
            CharMatch::DoesNotExist('A'),
            CharMatch::DoesNotExist('S'),
            CharMatch::DoesNotExist('T'),
        ];

        assert!(a.match_vector == match_vec);
    }


}
