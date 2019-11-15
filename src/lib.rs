extern crate pest;
///
/// ## Rhyme
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(true, ttaw::rhyme("here", "near"));
/// assert_eq!(false, ttaw::rhyme("shopping", "cart"));
/// ```
///
/// ## Alliteration
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(true, ttaw::alliteration("a group of bounding bears"));
/// assert_eq!(true, ttaw::alliteration("boucing bears are everywhere"));
/// assert_eq!(false, ttaw::alliteration("The quick brown fox jumps over the lazy dog."));
/// ```
///
/// ## Double Metaphone
/// ```rust
/// extern crate ttaw;
/// use ttaw;
///     assert_eq!(ttaw::double_metaphone("Arnow").primary, "ARN");
///     assert_eq!(ttaw::double_metaphone("Arnow").secondary, "ARNF");
///
///     assert_eq!(ttaw::double_metaphone("detestable").primary, "TTSTPL");
///     assert_eq!(ttaw::double_metaphone("detestable").secondary, "TTSTPL");
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
pub mod double_metaphone;

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
