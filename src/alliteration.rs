extern crate log;

use crate::pronounciation::double_metaphone;

pub fn alliteration(s: &str) -> bool {
    let words = s.split_whitespace().collect::<Vec<&str>>();
    let comparisons = words.iter().zip(words.get(1..).unwrap_or_default().iter());

    for (a, b) in comparisons {
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
            b,
            b_phonetic.primary,
            b_phonetic.secondary
        );

        let mut a_phonetic_head_primary = a_phonetic.primary;

        if let Some(c) = a_phonetic_head_primary.get(..1) {
            a_phonetic_head_primary = c.to_string();
        }

        let mut a_phonetic_head_secondary = a_phonetic.secondary;

        if let Some(c) = a_phonetic_head_secondary.get(..1) {
            a_phonetic_head_secondary = c.to_string();
        }

        let mut b_phonetic_head_primary = b_phonetic.primary;

        if let Some(c) = b_phonetic_head_primary.get(..1) {
            b_phonetic_head_primary = c.to_string();
        }

        let mut b_phonetic_head_secondary = b_phonetic.secondary;

        if let Some(c) = b_phonetic_head_secondary.get(..1) {
            b_phonetic_head_secondary = c.to_string();
        }

        // Need to think more about how this _should_ be applied to alliteration.
        // My ears don't hear alliteration in "where ant" or "jumps over"
        if a_phonetic_head_primary == b_phonetic_head_primary
            || a_phonetic_head_primary == b_phonetic_head_secondary
            || a_phonetic_head_secondary == b_phonetic_head_primary
            || a_phonetic_head_secondary == b_phonetic_head_secondary
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bouncing_bears() {
        assert!(alliteration("bouncing bears"));
        assert!(alliteration("a group of bounding bears"));
        assert!(alliteration("boucing bears are everywhere"));
    }

    #[test]
    fn also_ants() {
        assert!(alliteration("also ants"));
        assert!(alliteration("there were seals at the zoo, also ants"));
        assert!(alliteration("Also, ants were writing poetry."));
    }

    #[test]
    fn questionalbe() {
        assert!(alliteration("where ants"));
        assert!(alliteration("jumps over"));
    }

    #[test]
    fn quick_brown_fox() {
        assert!(!alliteration("brown fox"));
        assert!(!alliteration("The quick brown fox"));
        assert!(!alliteration("The quick brown fox jumps."));
    }
}
