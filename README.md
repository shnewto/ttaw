[![Build Status](https://travis-ci.org/shnewto/ttaw.svg?branch=master)](https://travis-ci.org/shnewto/ttaw)
[![Coverage Status](https://coveralls.io/repos/github/shnewto/ttaw/badge.svg?branch=master)](https://coveralls.io/github/shnewto/ttaw?branch=master)
[![Crates.io Version](https://img.shields.io/crates/v/ttaw.svg)](https://crates.io/crates/ttaw)
[![Crates.io](https://img.shields.io/crates/d/ttaw.svg)](https://crates.io/crates/ttaw)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

# ttaw
talking to a wall, a piecemeal natural language processing library.

## A couple caveats
- My implementation's aren't a sure thing :smile: If you encounter anything peculiar or unexpected, please raise an issue :heart: :heart:

## Functionality
- Determine if two words rhyme using the Double Metaphone phonetic encoding
- Determine if two words rhyme using CMUdict phonetic encoding

- Determine if two words alliterate using the Double Metaphone phonetic encoding
- Determine if two words alliterate using CMUdict phonetic encoding

- Determine if two words alliterate using a combination of the CMUdict and the Double Metaphone phonetic encoding
- Determine if two words alliterate using a combination of the CMUdict and the Double Metaphone phonetic encoding

- Get the CMUdict phonetic encoding of a word
- Get the Double Metaphone phonetic encoding of a word (port of [words/double-metahone](https://github.com/words/double-metaphone) library)


## Rhyme
```rust
extern crate ttaw;
use ttaw;

assert_eq!(Ok(true), ttaw::rhyme("far", "tar"));
assert_eq!(Ok(false), ttaw::rhyme("shopping", "cart"));

// Deviations in cmu and metaphone
assert_eq!(true, ttaw::metaphone::rhyme("hear", "near"));
assert_eq!(Ok(false), ttaw::cmu::rhyme("hear", "near"));
```

## Alliteration
```rust
extern crate ttaw;
use ttaw;
assert_eq!(Ok(true), ttaw::alliteration("bounding","bears"));
assert_eq!(Ok(false), ttaw::alliteration("lazy", "dog"));

assert_eq!(true, ttaw::metaphone::alliteration("bounding","bears"));
assert_eq!(Ok(true), ttaw::cmu::alliteration("bounding","bears"));
```


## CMUdict
```rust
extern crate ttaw;
use ttaw;
assert_eq!(
    ttaw::cmu::cmu("unearthed"),
    Ok(Some(vec![vec![
        "AH0".to_string(),
        "N".to_string(),
        "ER1".to_string(),
        "TH".to_string(),
        "T".to_string()
    ]]))
);
```

## Double Metaphone
```rust
extern crate ttaw;
use ttaw;
assert_eq!(ttaw::metaphone::double_metaphone("Arnow").primary, "ARN");
assert_eq!(ttaw::metaphone::double_metaphone("Arnow").secondary, "ARNF");

assert_eq!(
    ttaw::metaphone::double_metaphone("detestable").primary,
    "TTSTPL"
);
assert_eq!(
    ttaw::metaphone::double_metaphone("detestable").secondary,
    "TTSTPL"
);
```
