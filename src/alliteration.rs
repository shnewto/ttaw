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

        let mut b_phonetic_head_primary = b_phonetic.primary;

        if let Some(c) = b_phonetic_head_primary.get(..1) {
            b_phonetic_head_primary = c.to_string();
        }

        // Need to do some thinking about where and when to support secondary pronouciation.
        // For example, with my approach, jumps' secondary AMPS alliterates with over's secondary AFR...
        if a_phonetic_head_primary == b_phonetic_head_primary {
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
    fn quick_brown_fox() {
        assert!(!alliteration("brown fox"));
        assert!(!alliteration("The quick brown fox"));
        assert!(!alliteration(
            "The quick brown fox jumps over the lazy dog."
        ));
    }
}
