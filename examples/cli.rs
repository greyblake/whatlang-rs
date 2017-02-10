extern crate whatlang;

use std::io;
use whatlang::{detect_lang};

fn main() {
    let mut text = String::new();
    println!("Please enter a text: ");
    io::stdin().read_line(&mut text).expect("Failed to read line");

    let query = Query { text: &text };
    if let Some(result) = detect_lang(query) {
        println!("Language: {}", result.lang.to_code());
    } else {
        println!("Cannot recognize any language :(");
    }
}

