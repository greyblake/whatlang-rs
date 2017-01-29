# Whatlang

Natural language detection in Rust.

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

### Supported languages (there gonna be much more)
* Esperanto (epo)
* English (eng)
* Russian (rus)
* Mandarin (cmn)
* Spanish (spa)
* Portuguese (por)
* Bengali (ben)
* French (fra)
* German (deu)
* Ukrainian (ukr)
* Georgian (kat)
* Arabic (arb)
* Hindi (hin)
* Japanese (jpn)
* Hebrew (heb)
* Yiddish (ydd)
* Polish (pol)
* Amharic (ahm)
* Tigrinya (tir)
* Javanese (jav)
* Korean (kor)

## License

MIT

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer
