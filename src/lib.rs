extern crate pest;
///
/// ## Rhyme
/// ```rust
/// extern crate ttaw;
/// use ttaw::pronounciation;
/// assert_eq!(true, pronounciation::rhyme("here", "near"));
/// assert_eq!(false, pronounciation::rhyme("shopping", "cart"));
/// ```
///
/// ## Alliteration
/// ```rust
/// extern crate ttaw;
/// use ttaw::pronounciation;
/// assert_eq!(true, pronounciation::alliteration("a group of bounding bears"));
/// assert_eq!(true, pronounciation::alliteration("boucing bears are everywhere"));
/// assert_eq!(false, pronounciation::alliteration("The quick brown fox jumps over the lazy dog."));
/// ```
///
/// ## Double Metaphone
/// ```rust
/// extern crate ttaw;
/// use ttaw::pronounciation;
///     assert_eq!(pronounciation::double_metaphone("Arnow").primary, "ARN");
///     assert_eq!(pronounciation::double_metaphone("Arnow").secondary, "ARNF");
///
///     assert_eq!(pronounciation::double_metaphone("detestable").primary, "TTSTPL");
///     assert_eq!(pronounciation::double_metaphone("detestable").secondary, "TTSTPL");
/// ```
#[cfg(test)]
extern crate tempfile;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
mod error;
pub use error::Error;
pub mod cmu;
pub mod double_metaphone;
pub mod pronounciation;

pub fn rhyme(a: &str, b: &str) -> Result<bool, Error> {
    if cmu::rhyme(a, b)? {
        return Ok(true);
    }

    Ok(double_metaphone::rhyme(a, b))
}

pub fn alliteration(a: &str, b: &str) -> Result<bool, Error> {
    if cmu::alliteration(a, b)? {
        return Ok(true);
    }

    Ok(double_metaphone::alliteration(a, b))
}
