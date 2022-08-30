### v0.16.1 - 2022-08-30
* Fix bug in Czech alphabet (improved quality of Czech language detection)

### v0.16.0 - 2022-05-07
* [breaking] Add Armenian script (`Script::Armenian`) and language (`Lang::Hye`)

### v0.15.0 - 2022-05-01
* Update enum-map dependency to version 2
* Optimize alphabet method for Cyrillic: almost 2x improved performance for Cyrillic languages and 7% for the average `detect()` benchmark.

### v0.14.0 - 2022-04-15
* Improve performance of `detect()` almost twice ([see PR](https://github.com/greyblake/whatlang-rs/pull/108))

### v0.13.0 - 2022-01-02
* [breaking] - Support of Tagalog (`Tgl`)
* Rename `whatlang::Error` -> `whatlang::ParseError`

### v0.12.0 - 2021-04-18
* [breaking] - Drop languages:
  * Tigrinya (`Tir`)
  * Hausa (`Hau`)
  * Chewa (`Nya`)
  * Bhojpuri (`Bho`)
  * Igbo (`Ibo`)
  * Maithili (`Mai`)
  * Oromo (`Orm`)
  * Rundi (`Run`)
  * Saraiki (`Srk`)
  * Kurdish (`Kur`)
  * Cebuano (`Ceb`)
  * Malagasy (`Mlg`)
  * Kinyarwanda (`Kin`)
  * Somali (`Som`)
  * Ilocano (`Ilo`)
  * Uyghur (`Uig`)
  * Tagalog ('Tgl')
  * Haitian Creole (`Hat`)
  * Nynorsk (`Nno`)
  * Yoruba (`Yor`)
* [breaking] - Rename Yiddish: `Ydd` -> `Yid`
* [breaking] - Rename Azerbaijani: `Azj` -> `Aze`
* [breaking] Rename List -> FilterList
* [breaking] rename `whitelist` and `blacklist` to `allowlist` and `denylist` respectively
* Drop support of Cyrillic Azerbaijani and Turkmen
* Add `Script::all()` to iterate over all scripts.
* Add `Lang::all()` to iterate over all languages.
* Add integration with `enum-map`
* Implement `FromStr` for `Script` and `Lang`
* Implement `Script::langs(&self) -> &[Lang]`
* About 7% slower than v0.11.1 due to new detection method introduced. It's still much faster than v0.11.0

### v0.11.1 - 2020-11-28
* Use Trigram tuple instead of heap allocated String. (~68% faster)

### v0.11.0 - 2020-11-03
* [breaking] - rename code for Arabic: `Arb` -> `Ara`

### v0.10.0 - 2020-09-04
* Support Catalan

### v0.9.0 - 2020-06-26
* Support Slovak

### v0.8.0 - 2020-05-08
* Support Latin

### v0.7.4 - 2020-04-26 (yanked version)
* Support Latin

### v0.7.2 - 2019-10-19
* (fix) respect japanese whitelisting when mandarin characters are given (#44)

### v0.7.1 - 2019-05-06
* Update dependency hashbrown 0.1.8 -> 0.3.0 (10% faster)

### v0.7.0 - 2019-03-03
* Support Afrikaans language (afr)
* Get rid of build dependencies: installation is much faster now

### v0.6.0 - 2018-11-09
* Use hashbrown instead of fnv (detect() is 30% faster)
* Use array on stack instead of vector for detect_script (1-2% faster)
* Use build.rs to generate `lang.rs` file
* Add property based testing

### v0.5.0 - 2017-08-06
* (breaking) Rename `Lang::to_code(&self)` to `Lang::code(&self)`
* (fix) Fix bug with zero division in confidence calculation
* (fix) Confidence can not exceed 1.0
* Implement `Lang::eng_name(&self) -> &str` function
* Implement `Lang::name(&self) -> &str` function
* Implement `Script::name(&self) -> &str` function
* Implement trait `Dislpay` for `Script`
* Implement `Display` trait for `Lang`

### v0.4.1 - 2017-07-31
* Calculate confidence in the range from 0 to 1 for Info

### v0.4.0 - 2017-07-30
* Calculate is_reliable bool for `Info` struct.
* Breaking changes for `Info`. Make fields private. Now one should use methods.
* Remove support of Latin version of Serbo-Croatian, because it conflicts a lot with modern Croatian.

### v0.3.3 - 2017-07-26
* Replace HashMap with FnvHashMap (~ 33% faster)

### v0.3.2 - 2017-06-04
* Small performance improvement: preallocate memory for counter_hash in trigrams.rs (~ 2-3% faster)

### v0.3.1 - 2017-02-10
* Fix build
* Add link to doc at crates.io

### v0.3.0 - 2017-02-10
* Support New 14 languages
* (breaking) New API

### v0.2.1 - 2017-02-07
* Support 10 new languages
* Optimize trigram algorithms

### v0.2.0 - 2017-02-06
* Optimize script detection
* Accept text, blacklist and whitelist as references
* 10 new languages
* Fix: always guarantee same result on same input data (fix sorting issue)

### v0.1.4 - 2017-02-04
* Support whitelist and blacklist

### v0.1.3 - 2017-02-03
* Support more than 50 languages

### v0.1.2 - 2017-01-29
* Support about 20 languages

### v0.1.1 - 2016-12-25
* Tiny improvements

### v0.1.0 - 2016-12-25
* First public release
