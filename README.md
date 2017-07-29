# Whatlang

[![Build Status](https://travis-ci.org/greyblake/whatlang-rs.svg?branch=master)](https://travis-ci.org/greyblake/whatlang-rs) [DOCUMENTATION](https://docs.rs/whatlang).

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

Small example:

```rust
use whatlang::{detect, Lang, Script};

// Detect Esperanto (there are also `detect_lang` and `detect_script` functions)
let info = detect("Ĉu vi ne volas eklerni Esperanton? Bonvolu!").unwrap();
assert_eq!(info.lang, Lang::Epo);
assert_eq!(info.script, Script::Latin);
```

## Blacklisting and whitelisting

You can create configured detector to apply blacklist or whitelist:

```rust
use whatlang::{Detector, Lang};

const WHITELIST : &'static [Lang] = &[Lang::Eng, Lang::Rus];

// You can also create detector using `with_blacklist` function
let detector = Detector::with_whitelist(WHITELIST);

// There are also `detect` and `detect_script` functions
let lang = detector.detect_lang("There is no reason not to learn Esperanto.");
assert_eq!(lang, Some(Lang::Eng));
```

For more details, please check [documentation](https://docs.rs/whatlang/).

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
