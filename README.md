[![Build Status](https://travis-ci.org/shnewto/ttaw.svg?branch=master)](https://travis-ci.org/shnewto/ttaw)
[![Coverage Status](https://coveralls.io/repos/github/shnewto/ttaw/badge.svg?branch=master)](https://coveralls.io/github/shnewto/ttaw?branch=master)
[![Crates.io Version](https://img.shields.io/crates/v/ttaw.svg)](https://crates.io/crates/ttaw)
[![Crates.io](https://img.shields.io/crates/d/ttaw.svg)](https://crates.io/crates/ttaw)

# ttaw
talking to a wall, a piecemeal natural language processing library

## Functionality
- Determine if two words rhyme (perfect single or general syllabic).
- Determine if there exists consecutive alliteration in an &str.
- Double Metaphone phonetic encoding, port of [words/double-metahone](https://github.com/words/double-metaphone) library.


## Rhyme
```rust
extern crate ttaw;
use ttaw::pronounciation;
assert_eq!(true, pronounciation::rhyme("here", "near"));
assert_eq!(false, pronounciation::rhyme("shopping", "cart"));
```

## Alliteration
```rust
extern crate ttaw;
use ttaw::pronounciation;
assert_eq!(true, pronounciation::alliteration("a group of bounding bears"));
assert_eq!(true, pronounciation::alliteration("boucing bears are everywhere"));
assert_eq!(false, pronounciation::alliteration("The quick brown fox jumps over the lazy dog."));
```

## Double Metaphone
```rust
extern crate ttaw;
use ttaw::pronounciation;
    assert_eq!(pronounciation::double_metaphone("Arnow").primary, "ARN");
    assert_eq!(pronounciation::double_metaphone("Arnow").secondary, "ARNF");

    assert_eq!(pronounciation::double_metaphone("detestable").primary, "TTSTPL");
    assert_eq!(pronounciation::double_metaphone("detestable").secondary, "TTSTPL");
```
