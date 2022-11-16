mod common;
use std::collections::HashMap;

use common::{setup, teardown};
use word_counter::{counter, WordCounter};

#[test]
fn test_with_file() {
    setup();

    println!("{:?}", counter(&String::from("test.txt")));

    assert_eq!(
        counter(&String::from("test.txt")),
        WordCounter(HashMap::from([
            ("jumps".to_string(), 1),
            ("dog".to_string(), 1),
            ("quick".to_string(), 1),
            ("brown".to_string(), 1),
            ("the".to_string(), 2),
            ("lazy".to_string(), 1),
            ("fox".to_string(), 1),
            ("over".to_string(), 1),
        ]))
    );
    teardown().expect("Could not delete file");
}
