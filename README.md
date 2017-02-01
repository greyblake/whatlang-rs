# Whatlang

[![Build Status](https://travis-ci.org/greyblake/whatlang-rs.svg?branch=master)](https://travis-ci.org/greyblake/whatlang-rs)

Natural language detection in Rust.

## Features
* 100% written in Rust
* No dependencies
* Support about 30 most popular languages
* Recognizes not only a language, but also a script (Latin, Cyrillic, etc)

## Get started

The library is still in active development. Here is the short example how to use it:

Add to you `Cargo.toml`:
```
[dependencies]

whatlang = "*"
```

In you program:

```rust
extern crate whatlang;

use whatlang::{detect_lang, Lang, Script, Query};

fn main() {
    let text = "Guten Abend, meine Damen und Herren!".to_string();
    let query = Query::new(&text);
    let result = detect_lang(query).unwrap();
    assert_eq!(result.lang, Lang::Deu);
    assert_eq!(result.lang.to_code(), "deu");
    assert_eq!(result.script, Script::Latin);
}
```

## Roadmap

* Support 100 most popular languages
* Allow to specify whitelist/blacklist in `Query` struct
* Provide some metrics about reliability in `Result` struct
* Tune performance

### Supported languages

| Language   | ISO 639-3 | Enum        |
| ---------- | --------- | ----------- |
| Esperanto  | epo       | `Lang::Epo` |
| English    | eng       | `Lang::Eng` |
| Russian    | rus       | `Lang::Rus` |
| Mandarin   | cmn       | `Lang::Cmn` |
| Spanish    | spa       | `Lang::Spa` |
| Portuguese | por       | `Lang::Por` |
| Italian    | ita       | `Lang::Ita` |
| Bengali    | ben       | `Lang::Ben` |
| French     | fra       | `Lang::Fra` |
| German     | deu       | `Lang::Deu` |
| Ukrainian  | ukr       | `Lang::Ukr` |
| Georgian   | kat       | `Lang::Kat` |
| Arabic     | arb       | `Lang::Arb` |
| Hindi      | hin       | `Lang::Hin` |
| Japanese   | jpn       | `Lang::Jpn` |
| Hebrew     | heb       | `Lang::Heb` |
| Yiddish    | ydd       | `Lang::Ydd` |
| Polish     | pol       | `Lang::Pol` |
| Amharic    | ahm       | `Lang::Ahm` |
| Tigrinya   | tir       | `Lang::Tir` |
| Javanese   | jav       | `Lang::Jav` |
| Korean     | kor       | `Lang::Kor` |
| Bokmal     | nob       | `Lang::Nob` |
| Nynorsk    | nno       | `Lang::Nno` |
| Danish     | dan       | `Lang::Dan` |
| Swedish    | swe       | `Lang::Swe` |
| Finnish    | fin       | `Lang::Fin` |
| Turkish    | tur       | `Lang::Tur` |
| Dutch      | nld       | `Lang::Nld` |
| Hungarian  | hun       | `Lang::Hun` |
| Czech      | ces       | `Lang::Ces` |
| Greek      | ell       | `Lang::Ell` |

## License

MIT

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer
