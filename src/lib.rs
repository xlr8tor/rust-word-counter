//!This is a word counter crate built the count the number of occurrences
//! of words in a file

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
///Implements a tuple struct of
pub struct WordCounter(pub HashMap<String, u32>);

///Implements a new WordCounter instance
impl WordCounter {
    pub fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    ///Increment the count for each for occurrence in the file
    pub fn increment(&mut self, word: &str) {
        let word = String::from(word);
        let count = self.0.entry(word).or_insert(0);
        *count += 1;
    }

    ///Displays the words contained in WordCounter
    pub fn display(&self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value)
        }
    }
}

///Takes a file name as input and creates a file handle used to process the file
/// returns an instance of WordCounter
pub fn counter(mut file: &String) -> WordCounter {
    let file = File::open(&mut file).expect("Could not find file");
    let reader = BufReader::new(file);

    let mut word_count = WordCounter::new();
    for lines in reader.lines() {
        let lines = lines.expect("Could not read lines");
        let words = lines.split(" ");

        for word in words {
            if word == "" {
                continue;
            }
            word_count.increment(word);
        }
    }
    word_count.display();
    word_count
}

#[cfg(test)]
mod tests {
    use super::{HashMap, WordCounter};

    #[test]
    fn test_new() {
        assert_eq!(WordCounter::new(), WordCounter(HashMap::new()))
    }
}
