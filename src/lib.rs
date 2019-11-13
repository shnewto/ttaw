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
extern crate pest;
#[macro_use]
extern crate pest_derive;
mod cmu;
mod error;
pub use error::Error;
pub mod pronounciation;
