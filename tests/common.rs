use std::fs::{self, File};
use std::io;
use std::io::prelude::*;

pub fn setup() {
    let mut file = File::create("test.txt").expect("Could not create file");
    file.write_all("the quick brown fox jumps over the lazy dog".as_bytes())
        .unwrap();
}

pub fn teardown() -> Result<(), io::Error> {
    fs::remove_file("test.txt")?;
    Ok(())
}
