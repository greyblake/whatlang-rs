<p align="center"><img width="160" src="https://raw.githubusercontent.com/greyblake/whatlang-rs/master/misc/logo/whatlang-logo.svg" alt="Whatlang - rust library for natural language detection"></p>

<h1 align="center">Whatlang</h1>

<p align="center">Natural language detection for Rust with focus on simplicity and performance.</p>
<p align="center"><a href="https://whatlang.org/" target="_blank">Try online demo.</a></p>

<p align="center">
<a href="https://github.com/greyblake/whatlang-rs/actions/workflows/ci.yml" rel="nofollow"><img src="https://github.com/greyblake/whatlang-rs/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
<a href="https://raw.githubusercontent.com/greyblake/whatlang-rs/master/LICENSE" rel="nofollow"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
<a href="https://docs.rs/whatlang" rel="nofollow"><img src="https://docs.rs/whatlang/badge.svg" alt="Documentation"></a>
<p>

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua/)

## Content
* [Features](#features)
* [Get started](#get-started)
* [Who uses Whatlang?](#who-uses-whatlang)
* [Documentation](https://docs.rs/whatlang)
* [Supported languages](https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md)
* [Feature toggles](#feature-toggles)
* [How does it work?](#how-does-it-work)
  * [How language recognition works?](#how-language-recognition-works)
  * [How is_reliable calculated?](#how-is_reliable-calculated)
* [Running benchmark](#running-benchmarks)
* [Comparison with alternatives](#comparison-with-alternatives)
* [Ports and clones](#ports-and-clones)
* [Donations](#donations)
* [Derivation](#derivation)
* [License](#license)
* [Contributors](#contributors)


## Features
* Supports [69 languages](https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md)
* 100% written in Rust
* Lightweight, fast and simple
* Recognizes not only a language, but also a script (Latin, Cyrillic, etc)
* Provides reliability information

## Get started

Example:

```rust
use whatlang::{detect, Lang, Script};

fn main() {
    let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";

    let info = detect(text).unwrap();
    assert_eq!(info.lang(), Lang::Epo);
    assert_eq!(info.script(), Script::Latin);
    assert_eq!(info.confidence(), 1.0);
    assert!(info.is_reliable());
}
```

For more details (e.g. how to blacklist some languages) please check the [documentation](https://docs.rs/whatlang).

## Who uses Whatlang?

Whatlang is used within the following big projects as direct or indirect dependency for language recognition.
You're gonna be in a great company using Whatlang:

* [Sonic](https://github.com/valeriansaliou/sonic) - fast, lightweight and schema-less search backend in Rust.
* [Meilisearch](https://github.com/meilisearch) - an open-source, easy-to-use, blazingly fast, and hyper-relevant search engine built in Rust.

## Feature toggles

| Feature    | Description                                                                           |
|------------|---------------------------------------------------------------------------------------|
| `enum-map` | `Lang` and `Script` implement `Enum` trait from [enum-map](https://docs.rs/enum-map/) |
| `dev`      | Enables `whatlang::dev` module which provides some internal API.<br/> It exists for profiling purposes and normal users are discouraged to to rely on this API.  |

## How does it work?

### How does the language recognition work?

The algorithm is based on the trigram language models, which is a particular case of n-grams.
To understand the idea, please check the original whitepaper [Cavnar and Trenkle '94: N-Gram-Based Text Categorization'](https://www.researchgate.net/publication/2375544_N-Gram-Based_Text_Categorization).

### How is `is_reliable` calculated?

It is based on the following factors:
* How many unique trigrams are in the given text
* How big is the difference between the first and the second(not returned) detected languages? This metric is called `rate` in the code base.

Therefore, it can be presented as 2d space with threshold functions, that splits it into "Reliable" and "Not reliable" areas.
This function is a hyperbola and it looks like the following one:

<img alt="Language recognition whatlang rust" src="https://raw.githubusercontent.com/greyblake/whatlang-rs/master/misc/images/whatlang_is_reliable.png" width="450" height="300" />

For more details, please check a blog article [Introduction to Rust Whatlang Library and Natural Language Identification Algorithms](https://www.greyblake.com/blog/introduction-to-rust-whatlang-library-and-natural-language-identification-algorithms/).

## Make tasks

* `make bench` - run performance benchmarks
* `make doc` - generate and open doc
* `make test` - run tests
* `make watch` - watch changes and run tests

## Comparison with alternatives

|                           | Whatlang   | CLD2        | CLD3           |
| ------------------------- | ---------- | ----------- | -------------- |
| Implementation language   | Rust       | C++         | C++            |
| Languages                 | 68         | 83          | 107            |
| Algorithm                 | trigrams   | quadgrams   | neural network |
| Supported Encoding        | UTF-8      | UTF-8       | ?              |
| HTML support              | no         | yes         | ?              |


## Ports and clones

* [whatlang-ffi](https://github.com/greyblake/whatlang-ffi) - C bindings
* [whatlanggo](https://github.com/abadojack/whatlanggo) - whatlang clone for Go language
* [whatlang-py](https://github.com/cathalgarvey/whatlang-py) - bindings for Python
* [whatlang-rb](https://gitlab.com/KitaitiMakoto/whatlang-rb) - bindings for Ruby

## Donations

You can support the project by donating [NEAR tokens](https://near.org).

Our NEAR wallet address is `whatlang.near`

## Derivation

**Whatlang** is a derivative work from [Franc](https://github.com/wooorm/franc) (JavaScript, MIT) by [Titus Wormer](https://github.com/wooorm).

## License

[MIT](https://github.com/greyblake/whatlang-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com/)


## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [Dr-Emann](https://github.com/Dr-Emann) Zachary Dremann - optimization and improvements
- [BaptisteGelez](https://github.com/BaptisteGelez) Baptiste Gelez - improvements
- [Vishesh Chopra](https://github.com/KarmicKonquest) - designed the logo
- [Joel Natividad](https://github.com/jqnatividad) - support of Tagalog
- [ManyTheFish](https://github.com/ManyTheFish) - crazy optimization
- [Kerollmops](https://github.com/Kerollmops) Clément Renault - crazy optimization
