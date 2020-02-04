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
extern crate reqwest;
extern crate serde_json;
mod error;
pub use error::Error;
pub mod cmu;
pub mod metaphone;
