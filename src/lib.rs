extern crate pest;
///
/// ## Rhyme
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(Ok(true), ttaw::rhyme("here", "near"));
/// assert_eq!(Ok(false), ttaw::rhyme("shopping", "cart"));
/// ```
///
/// ## Alliteration
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(Ok(true), ttaw::alliteration("bounding", "bears"));
/// assert_eq!(Ok(true), ttaw::alliteration("snappy", "snails"));
/// assert_eq!(Ok(false), ttaw::alliteration("lazy", "dog"));
/// ```
///
/// ## Double Metaphone
/// ```rust
/// extern crate ttaw;
/// use ttaw;
///     assert_eq!(ttaw::metaphone::double_metaphone("Arnow").primary, "ARN");
///     assert_eq!(ttaw::metaphone::double_metaphone("Arnow").secondary, "ARNF");
///
///     assert_eq!(ttaw::metaphone::double_metaphone("detestable").primary, "TTSTPL");
///     assert_eq!(ttaw::metaphone::double_metaphone("detestable").secondary, "TTSTPL");
/// ```
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
mod error;
pub use error::Error;
pub mod cmu;
pub mod metaphone;

/// Use CMUdict and the Double Metahone phonetic encoding to determine if two words rhyme. If the CMUdict check doesn't find a rhyme, the Double Metahone encoding is evaluated for rhyme.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// // Does rhyme
/// assert_eq!(Ok(true), ttaw::rhyme("hissed", "mist"));
/// assert_eq!(Ok(true), ttaw::rhyme("tryst", "wrist"));
///
/// // Does not rhyme
/// assert_eq!(Ok(false), ttaw::rhyme("red", "edmund"));
/// assert_eq!(Ok(false), ttaw::rhyme("comfy", "chair"));
/// ```
pub fn rhyme(a: &str, b: &str) -> Result<bool, Error> {
    if cmu::rhyme(a, b)? {
        return Ok(true);
    }

    Ok(metaphone::rhyme(a, b))
}

/// Use CMUdict and the Double Metahone phonetic encoding to determine if two words alliterate. If the CMUdict check doesn't find a alliteration, the Double Metahone encoding is evaluated for alliteration.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
// // Does alliterate
/// assert_eq!(Ok(true), ttaw::alliteration("bouncing", "bears"));
/// assert_eq!(Ok(true), ttaw::alliteration("snappy", "snails"));
///
/// // Does not alliterate
/// assert_eq!(Ok(false), ttaw::alliteration("brown", "fox"));
/// assert_eq!(Ok(false), ttaw::alliteration("lazy", "dog"));
/// ```
pub fn alliteration(a: &str, b: &str) -> Result<bool, Error> {
    if cmu::alliteration(a, b)? {
        return Ok(true);
    }

    Ok(metaphone::alliteration(a, b))
}
