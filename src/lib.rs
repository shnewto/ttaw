extern crate pest;
///
/// ## CMU
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// let cmudict = ttaw::cmu::CmuDict::new("cmudict.test").unwrap();
/// assert_eq!(
///     cmudict.encoding("permeability"),
///     Ok(Some(vec![vec![
///         "P".to_string(),
///         "ER0".to_string(),
///         "M".to_string(),
///         "IY2".to_string(),
///         "AH0".to_string(),
///         "B".to_string(),
///         "IH1".to_string(),
///         "L".to_string(),
///         "IH0".to_string(),
///         "T".to_string(),
///         "IY0".to_string()
///     ]]))
/// );
/// assert_eq!(
///     cmudict.encoding("unearthed"),
///     Ok(Some(vec![vec![
///         "AH0".to_string(),
///         "N".to_string(),
///         "ER1".to_string(),
///         "TH".to_string(),
///         "T".to_string()
///     ]]))
/// );
///
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
