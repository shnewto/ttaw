extern crate log;

use crate::pronounciation::double_metaphone;

pub fn rhymes(a: &str, b: &str) -> bool {
    // sanity check, needing to sanity check seems fragile?
    if a.trim().is_empty() || b.trim().is_empty() {
        return false;
    }

    let a_phonetic = double_metaphone(a);
    let b_phonetic = double_metaphone(b);

    log::info!(
        "|{: ^10} | {: ^10} | {: ^10} |",
        a,
        a_phonetic.primary,
        a_phonetic.secondary
    );
    log::info!(
        "|{: ^10} | {: ^10} | {: ^10} |",
        a,
        b_phonetic.primary,
        b_phonetic.secondary
    );

    let mut a_phonetic_end_primary = a_phonetic.primary;
    if let Some(slice) = a_phonetic_end_primary.get(1..) {
        a_phonetic_end_primary = slice.to_string();
    }

    let mut a_phonetic_end_secondary = a_phonetic.secondary;

    if let Some(slice) = a_phonetic_end_secondary.get(1..) {
        a_phonetic_end_secondary = slice.to_string();
    }

    let mut b_phonetic_end_primary = b_phonetic.primary;

    if let Some(slice) = b_phonetic_end_primary.get(1..) {
        b_phonetic_end_primary = slice.to_string();
    }

    let mut b_phonetic_end_secondary = b_phonetic.secondary;

    if let Some(slice) = b_phonetic_end_secondary.get(1..) {
        b_phonetic_end_secondary = slice.to_string();
    }

    a_phonetic_end_primary == b_phonetic_end_primary
        || a_phonetic_end_primary == b_phonetic_end_secondary
        || a_phonetic_end_secondary == b_phonetic_end_primary
        || a_phonetic_end_secondary == b_phonetic_end_secondary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_rhyme() {
        assert!(rhymes("here", "near"));
        assert!(rhymes("far", "tar"));
        assert!(rhymes("a", "say"));
        assert!(rhymes("dissed", "mist"));
    }

    #[test]
    fn do_not_rhyme() {
        assert!(!rhymes("tryst", "wrist"));
        assert!(!rhymes("dissed", "trust"));
        assert!(!rhymes("red", "Edmund"));
        assert!(!rhymes("shopping", "cart"));
        assert!(!rhymes("run", "uphill"));
        assert!(!rhymes("comfy", "chair"));

        assert!(!rhymes("empty", "  "));
        assert!(!rhymes("empty", ""));
        assert!(!rhymes("empty", "\t"));
        assert!(!rhymes("empty", "\r"));
        assert!(!rhymes("empty", "\n"));
    }
}
