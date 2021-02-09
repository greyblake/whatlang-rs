extern crate whatlang;

use std::io;
use whatlang::detect;

fn main() {
    let mut text = String::new();
    println!("Please enter a text:");
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    if let Some(info) = detect(&text) {
        println!("Language: {}", info.lang());
        println!("Info: {:?}", info);
    } else {
        println!("Cannot recognize a language :(");
    }
}
