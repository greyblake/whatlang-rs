extern crate skeptic;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::process::Command;

const DATA_PATH: &'static str = "misc/data.json";
const SUPPORTED_LANG_PATH: &'static str = "misc/supported_languages.csv";
const RUBY_HELPER: &'static str = "misc/update_support_languages.rb";
const TRIGRAM_COUNT: usize = 300;

fn main() {
    println!("cargo:rerun-if-changed={}", DATA_PATH);
    println!("cargo:rerun-if-changed={}", SUPPORTED_LANG_PATH);

    generate_source_files();
    skeptic::generate_doc_tests(&["README.md"]);
}

fn generate_source_files() {
	let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lang.rs");
	let mut lang_def = BufWriter::new(File::create(&dest_path).unwrap());

	println!("GENERATING CODE");
    let generated_code = Command::new("ruby")
    	.arg(RUBY_HELPER)
    	.output()
    	.expect("Failed to generate language metadata")
    	.stdout;

	println!("CODE GEN {:?} \n\n\n", generated_code);
    lang_def.write(&generated_code).unwrap();
}
