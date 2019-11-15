use cmu;
use double_metaphone;
use error::Error;
///
/// Determine if two words rhyme using a combination of CMU dictionary lookups and double-metaphone pronounciations.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw::pronounciation;
/// assert_eq!(true, pronounciation::rhyme("here", "near"));
/// assert_eq!(false, pronounciation::rhyme("shopping", "cart"));
/// ```
///
pub fn rhyme(a: &str, b: &str) -> Result<bool, Error> {
    if cmu::rhyme(a, b)? {
        return Ok(true);
    }

    Ok(double_metaphone::rhyme(a, b))
}

///
/// Determine if there exists consecutive alliteration in an &str.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw::pronounciation;
/// assert_eq!(true, pronounciation::alliteration("a group of bounding bears"));
/// assert_eq!(true, pronounciation::alliteration("boucing bears are everywhere"));
/// assert_eq!(false, pronounciation::alliteration("The quick brown fox jumps over the lazy dog."));
/// ```
///
pub fn alliteration(s: &str) -> bool {
    double_metaphone::alliteration(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_single() {
        assert!(rhyme("far", "tar").unwrap());
        assert!(rhyme("a", "say").unwrap());
        assert!(rhyme("hissed", "mist").unwrap());
        assert!(rhyme("dissed", "mist").unwrap());
        assert!(rhyme("tryst", "wrist").unwrap());
        assert!(rhyme("here", "near").unwrap());
    }

    #[test]
    fn no_rhyme() {
        assert!(!rhyme("dissed", "trust").unwrap());
        assert!(!rhyme("red", "Edmund").unwrap());
        assert!(!rhyme("shopping", "cart").unwrap());
        assert!(!rhyme("run", "uphill").unwrap());
        assert!(!rhyme("comfy", "chair").unwrap());

        assert!(!rhyme("empty", "  ").unwrap());
        assert!(!rhyme("empty", "").unwrap());
        assert!(!rhyme("empty", "\t").unwrap());
        assert!(!rhyme("empty", "\r").unwrap());
        assert!(!rhyme("empty", "\n").unwrap());
    }

    #[test]
    fn general_syllabic() {
        assert!(rhyme("cleaver", "silver").unwrap());
        assert!(rhyme("pitter", "patter").unwrap());
        assert!(rhyme("bottle", "fiddle").unwrap());
    }
}
