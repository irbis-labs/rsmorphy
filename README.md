![](https://img.shields.io/crates/l/rsmorphy.svg)
[![crates.io](https://img.shields.io/crates/v/rsmorphy.svg)](https://crates.io/crates/rsmorphy)
[![Build Status](https://travis-ci.org/alexander-irbis/rsmorphy.svg)](https://travis-ci.org/alexander-irbis/rsmorphy)

## RsMorphy

Morphological analyzer (POS tagger + inflection engine) for Russian and Ukrainian languages.

RsMorphy is a native Rust implementation of [PyMorphy2](https://github.com/kmike/pymorphy2).


### Unstable

> **WIP. Very buggy.**

The implementation is at a very early stage and the API is a subject of changes.

__Note that RsMorphy currently requires the nightly version of the Rust compiler.__


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
