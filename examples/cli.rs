extern crate whatlang;

use std::io;
use whatlang::{detect_lang};

fn main() {
    let mut text = String::new();
    println!("Please enter a text: ");
    io::stdin().read_line(&mut text).expect("Failed to read line");

    if let Some(lang) = detect_lang(&text) {
        println!("Language: {}", lang.to_code());
    } else {
        println!("Cannot recognize a language :(");
    }
}

