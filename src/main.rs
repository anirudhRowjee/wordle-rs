extern crate colored;
use colored::*;
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
}

impl Attempt {

    fn new(length: usize) -> Attempt {

        Attempt {
            length,
            attempt_text: String::new(),
            match_vector: Vec::new(),
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

    fn resolve(&mut self, target_string: String) {

        let mut match_vector: Vec<CharMatch> = Vec::new();

        for (i, c) in self.attempt_text.chars().enumerate() {

            if target_string.contains(c) {
                // check if the position is right
                let index_check = target_string.to_string().as_bytes()[i];
                if (index_check as char) == c {
                    match_vector.push(CharMatch::ExistsHere(c));
                } else {
                    match_vector.push(CharMatch::ExistsSomewhereElse(c));
                }
            } else {
                match_vector.push(CharMatch::ExistsSomewhereElse(c));
            }
        }

        // println!("{}", match_vector);
        self.match_vector = match_vector;
        self.match_vector.iter().map(|x| println!("{:?}", x)).last();
    }

    // TODO figure out how to test this
    fn render(self) {
        println!("Entering Match Phase");
        self.match_vector
            .iter()
            .map(|x| {
                match x {
                    &CharMatch::DoesNotExist(char) => {
                        print!("{}", char.to_string().on_black().white());
                    }
                    &CharMatch::ExistsSomewhereElse(char) => {
                        print!("{}", char.to_string().on_yellow().black());
                    }
                    &CharMatch::ExistsHere(char) => {
                        print!("{}", char.to_string().on_green().black());
                    }
                };
            })
            .last();
        print!("\n");

    }
}

/// Hold all Game Data
struct Game {
    current_attempts: i32,
    max_attempts: i32,

    attempts: Vec<Attempt>,

    target_word: String,
    words: HashSet<String>,
}

fn main() {
    // let mut words = include_str!("./words.txt");

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
}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_attempt() {
        let target_text = String::from("FIERY");

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

        a.resolve(target_text);

        // manually match the vectors
        let match_vec = vec![
            CharMatch::ExistsHere('F'),
            CharMatch::ExistsSomewhereElse('E'),
            CharMatch::DoesNotExist('A'),
            CharMatch::DoesNotExist('S'),
            CharMatch::DoesNotExist('T'),
        ];

        // TODO figure out how to implement vector equality

        // assert!(a.match_vector == match_vec);
        // match_vec.iter().zip(a.match_vector).

    }


}
