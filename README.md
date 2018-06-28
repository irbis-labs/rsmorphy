![](https://img.shields.io/crates/l/rsmorphy.svg)
[![crates.io](https://img.shields.io/crates/v/rsmorphy.svg)](https://crates.io/crates/rsmorphy)

[![Build Status](https://travis-ci.org/irbis-labs/rsmorphy.svg)](https://travis-ci.org/irbis-labs/rsmorphy)
[![Coverage Status](https://coveralls.io/repos/github/irbis-labs/rsmorphy/badge.svg?branch=master)](https://coveralls.io/github/irbis-labs/rsmorphy?branch=master)
![Minimal rust version 1.26](https://img.shields.io/badge/rustc-1.27+-green.svg)
![Nightly rust version from May 13, 2018](https://img.shields.io/badge/rustc-nightly_2018--06--27-yellow.svg)

[![Join the chat at https://gitter.im/rsmorphy/Lobby](https://badges.gitter.im/rsmorphy/Lobby.svg)](https://gitter.im/rsmorphy/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Waffle.io - Columns and their card count](https://badge.waffle.io/irbis-labs/rsmorphy.svg?columns=inbox,backlog,in%20progress,done)](https://waffle.io/irbis-labs/rsmorphy)

## RsMorphy

Morphological analyzer (POS tagger + inflection engine) for Russian and Ukrainian languages.

RsMorphy is a native Rust implementation of [PyMorphy2](https://github.com/kmike/pymorphy2).


### Unstable

> **Work In Progress.**

The implementation is at a very early stage and the API is a subject of changes.

## Examples

### Inflection and plural

```bash
cargo run --example inflect
```
```
1 яблоко + 4 яблока = 5 яблок
102 яблока - 11 яблок = 91 яблоко
1 яблоком сыт не будешь
накормил 2 хлебами
```


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
