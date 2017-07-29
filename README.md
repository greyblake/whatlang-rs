# Whatlang

[![Build Status](https://travis-ci.org/greyblake/whatlang-rs.svg?branch=master)](https://travis-ci.org/greyblake/whatlang-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/greyblake/whatlang-rs/master/LICENSE)
[![Documentation](https://docs.rs/whatlang/badge.svg)](https://docs.rs/whatlang)

Natural language detection for Rust with focus on simplicity and performance.

## Features
* Supports [84 languages](https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md)
* 100% written in Rust
* No external dependencies (apart from [fnv](https://crates.io/crates/fnv) hasher)
* Fast
* Recognizes not only a language, but also a script (Latin, Cyrillic, etc)
* Provides reliability information

## Get started

Add to you `Cargo.toml`:
```
[dependencies]

whatlang = "0.3.3"
```

Example:

```rust
use whatlang::{detect, Lang, Script};

let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
let info = detect(text).unwrap();
assert_eq!(info.lang(), Lang::Epo);
assert_eq!(info.script(), Script::Latin);
assert!(info.is_reliable());
```

For more details (e.g. how to blacklist some languages) please check the [documentation](https://docs.rs/whatlang).

## Running benchmarks

```
cargo bench
```

## Ports and clones

* [whatlanggo](https://github.com/abadojack/whatlanggo) - whatlang clone for Go language

## Derivation

**Whatlang** is a derivative work from [Franc](https://github.com/wooorm/franc) (JavaScript, MIT) by [Titus Wormer](https://github.com/wooorm).

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [Dr-Emann](https://github.com/Dr-Emann) Zachary Dremann - optimization and improvements
