# Whatlang

[![Build Status](https://travis-ci.org/greyblake/whatlang-rs.svg?branch=master)](https://travis-ci.org/greyblake/whatlang-rs)

Natural language detection in Rust.

## Features
* Supports 75 languages
* 100% written in Rust
* No external dependencies
* Fast
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
    let text = "Guten Abend, meine Damen und Herren!";
    let query = Query::new(text);
    let result = detect_lang(query).unwrap();
    assert_eq!(result.lang, Lang::Deu);
    assert_eq!(result.lang.to_code(), "deu");
    assert_eq!(result.script, Script::Latin);
}
```

## Blacklist

Your can blacklist undesired languages, passing a vector.
In the example blow English and Spanish will be ignored:

```rust
let list = [Lang::Eng, Lang::Spa];
let query = Query::new(&text).blacklist(&list);
```

## Whitelist

In similar way, you can whitelist specified languages.
In this example, the library will recognize only Esperanto and Russian.
Note, if it detects a script that is different from Latin(Esperanto)
or Cyrillic(Russian), e.g. Greek, it will return `None`.

```rust
let list = [Lang::Epo, Lang::Rus];
let query = Query::new(&text).whitelist(&list);
```

## Roadmap

* Support 100 most popular languages
* ~~Allow to specify blacklist for Query~~
* ~~Allow to specify whitelist for Query~~
* [Support new API](https://github.com/greyblake/whatlang-rs/issues/5)
* Write doc for public structures and functions
* Improve README example
* Tune performance
* Create demo application
* Provide some metrics about reliability in `Result` struct


### Supported languages

| Language    | ISO 639-3 | Enum        |
| ----------- | --------- | ----------- |
| Esperanto   | epo       | `Lang::Epo` |
| English     | eng       | `Lang::Eng` |
| Russian     | rus       | `Lang::Rus` |
| Mandarin    | cmn       | `Lang::Cmn` |
| Spanish     | spa       | `Lang::Spa` |
| Portuguese  | por       | `Lang::Por` |
| Italian     | ita       | `Lang::Ita` |
| Bengali     | ben       | `Lang::Ben` |
| French      | fra       | `Lang::Fra` |
| German      | deu       | `Lang::Deu` |
| Ukrainian   | ukr       | `Lang::Ukr` |
| Georgian    | kat       | `Lang::Kat` |
| Arabic      | arb       | `Lang::Arb` |
| Hindi       | hin       | `Lang::Hin` |
| Japanese    | jpn       | `Lang::Jpn` |
| Hebrew      | heb       | `Lang::Heb` |
| Yiddish     | ydd       | `Lang::Ydd` |
| Polish      | pol       | `Lang::Pol` |
| Amharic     | amh       | `Lang::Amh` |
| Tigrinya    | tir       | `Lang::Tir` |
| Javanese    | jav       | `Lang::Jav` |
| Korean      | kor       | `Lang::Kor` |
| Bokmal      | nob       | `Lang::Nob` |
| Nynorsk     | nno       | `Lang::Nno` |
| Danish      | dan       | `Lang::Dan` |
| Swedish     | swe       | `Lang::Swe` |
| Finnish     | fin       | `Lang::Fin` |
| Turkish     | tur       | `Lang::Tur` |
| Dutch       | nld       | `Lang::Nld` |
| Hungarian   | hun       | `Lang::Hun` |
| Czech       | ces       | `Lang::Ces` |
| Greek       | ell       | `Lang::Ell` |
| Bulgarian   | bul       | `Lang::Bul` |
| Belarusian  | bel       | `Lang::Bel` |
| Marathi     | mar       | `Lang::Mar` |
| Kannada     | kan       | `Lang::Kan` |
| Romanian    | ron       | `Lang::Ron` |
| Slovene     | slv       | `Lang::Slv` |
| Croatian    | hrv       | `Lang::Hrv` |
| Serbian     | srp       | `Lang::Srp` |
| Macedonian  | mkd       | `Lang::Mkd` |
| Lithuanian  | lit       | `Lang::Lit` |
| Latvian     | lav       | `Lang::Lav` |
| Estonian    | est       | `Lang::Est` |
| Tamil       | tam       | `Lang::Tam` |
| Vietnamese  | vie       | `Lang::Vie` |
| Urdu        | urd       | `Lang::Urd` |
| Thai        | tha       | `Lang::Tha` |
| Gujarati    | guj       | `Lang::Guj` |
| Uzbek       | uzb       | `Lang::Uzb` |
| Punjabi     | pan       | `Lang::Pan` |
| Azerbaijani | azj       | `Lang::Azj` |
| Indonesian  | ind       | `Lang::Ind` |
| Telugu      | tel       | `Lang::Tel` |
| Persian     | pes       | `Lang::Pes` |
| Malayalam   | mal       | `Lang::Mal` |
| Hausa       | hau       | `Lang::Hau` |
| Oriya       | ori       | `Lang::Ori` |
| Burmese     | mya       | `Lang::Mya` |
| Bhojpuri    | bho       | `Lang::Bho` |
| Tagalog     | tgl       | `Lang::Tgl` |
| Yoruba      | yor       | `Lang::Yor` |
| Maithili    | mai       | `Lang::Mai` |
| Oromo       | orm       | `Lang::Orm` |
| Igbo        | ibo       | `Lang::Ibo` |
| Cebuano     | ceb       | `Lang::Ceb` |
| Kurdish     | kur       | `Lang::Kur` |
| Malagasy    | mlg       | `Lang::Mlg` |
| Saraiki     | skr       | `Lang::Skr` |
| Nepali      | nep       | `Lang::Nep` |
| Sinhalese   | sin       | `Lang::Sin` |
| Khmer       | khm       | `Lang::Khm` |
| Turkmen     | tuk       | `Lang::Tuk` |
| Somali      | som       | `Lang::Som` |
| Chewa       | nya       | `Lang::Nya` |

### Missing languages

The language that I did not find trigrams for:
* [Fula](https://en.wikipedia.org/wiki/Fula_language)

## License

MIT

## Acknowledgments

* Thanks [Franc JS](https://github.com/wooorm/franc) for trigrams dataset.

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer.
- [Dr-Emann](https://github.com/Dr-Emann) Zachary Dremann - optimization and improvements
