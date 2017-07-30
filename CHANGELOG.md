#### v0.4.1 - 2017-07-31
* Calculate confidence in the range from 0 to 1 for Info

#### v0.4.0 - 2017-07-30
* Calculate is_reliable bool for `Info` struct.
* Breaking changes for `Info`. Make fields private. Now one should use methods.
* Remove support of Latin version of Serbo-Croatian, because it conflicts a lot with modern Croatian.

#### v0.3.3 - 2017-07-26
* Replace HashMap with FnvHashMap (~ 33% faster)

#### v0.3.2 - 2017-06-04
* Small perfomance improvement: preallocate memory for counter_hash in trigrams.rs (~ 2-3% faster)

#### v0.3.1 - 2017-02-10
* Fix build
* Add link to doc at crates.io

#### v0.3.0 - 2017-02-10
* Support New 14 languages
* (breaking) New API

#### v0.2.1 - 2017-02-07
* Support 10 new languages
* Optimize trigram algorithms

#### v0.2.0 - 2017-02-06
* Optimize script detection
* Accept text, blacklist and whitelist as references
* 10 new languages
* Fix: always guarantee same result on same input data (fix sorting issue)

#### v0.1.4 - 2017-02-04
* Support whitelist and blacklist

#### v0.1.3 - 2017-02-03
* Support more than 50 languages

#### v0.1.2 - 2017-01-29
* Support about 20 languages

#### v0.1.1 - 2016-12-25
* Tiny improvements

#### v0.1.0 - 2016-12-25
* First public release
