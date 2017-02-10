#[macro_use]
extern crate bencher;
extern crate whatlang;
extern crate rustc_serialize;

use bencher::Bencher;
use rustc_serialize::json;
use std::collections::HashMap;
use whatlang::{detect, detect_script};

fn bench_detect(bench: &mut Bencher) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = json::decode(example_data).unwrap();

    bench.iter(|| {
        for (_, text) in &examples {
            detect(&text);
        }
    })
}

fn bench_detect_script(bench: &mut Bencher) {
    let example_data = include_str!("../tests/examples.json");
    let examples: HashMap<String, String> = json::decode(example_data).unwrap();

    bench.iter(|| {
        for (_, text) in &examples {
            detect_script(&text);
        }
    })
}

benchmark_group!(benches, bench_detect, bench_detect_script);
benchmark_main!(benches);
