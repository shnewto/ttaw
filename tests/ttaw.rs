extern crate ttaw;

use ttaw::{alliteration, rhyme};

#[test]
fn readme() {
    assert_eq!(Ok(true), ttaw::rhyme("far", "tar"));
    assert_eq!(Ok(false), ttaw::rhyme("shopping", "cart"));

    // Deviations in cmu and metaphone
    assert_eq!(true, ttaw::metaphone::rhyme("hear", "near"));
    assert_eq!(Ok(false), ttaw::cmu::rhyme("hear", "near"));

    assert_eq!(Ok(true), ttaw::alliteration("bounding", "bears"));
    assert_eq!(Ok(false), ttaw::alliteration("lazy", "dog"));

    assert_eq!(true, ttaw::metaphone::alliteration("bounding", "bears"));
    assert_eq!(Ok(true), ttaw::cmu::alliteration("bounding", "bears"));

    assert_eq!(
        ttaw::cmu::cmu("unearthed"),
        Ok(Some(vec![vec![
            "AH0".to_string(),
            "N".to_string(),
            "ER1".to_string(),
            "TH".to_string(),
            "T".to_string()
        ]]))
    );

    assert_eq!(ttaw::metaphone::double_metaphone("Arnow").primary, "ARN");
    assert_eq!(ttaw::metaphone::double_metaphone("Arnow").secondary, "ARNF");

    assert_eq!(
        ttaw::metaphone::double_metaphone("detestable").primary,
        "TTSTPL"
    );
    assert_eq!(
        ttaw::metaphone::double_metaphone("detestable").secondary,
        "TTSTPL"
    );
}

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

#[test]
fn alliterates_with_spaces() {
    assert!(alliteration("bouncing", "  bears").unwrap());
    assert!(alliteration("bouncing", "bears  ").unwrap());
    assert!(alliteration(" bouncing", "bears").unwrap());
    assert!(alliteration("bouncing  ", "bears").unwrap());
}

#[test]
fn alliterates_with_caps() {
    assert!(alliteration("Bouncing", "  bears").unwrap());
    assert!(alliteration("bouncing", "Bears  ").unwrap());
    assert!(alliteration(" bouncinG", "bEars").unwrap());
    assert!(alliteration("bouncing  ", "beaRs").unwrap());
}

#[test]
fn alliterates() {
    assert!(alliteration("bouncing", "bears").unwrap());
    assert!(alliteration("bounding", "bears").unwrap());
    assert!(alliteration("snappy", "snails").unwrap());
}

#[test]
fn quick_brown_fox() {
    assert!(!alliteration("where", "ants").unwrap());

    assert!(!alliteration("The", "quick").unwrap());
    assert!(!alliteration("brown", "fox").unwrap());
    assert!(!alliteration("jumps", "over").unwrap());
    assert!(!alliteration("a", "lazy").unwrap());
    assert!(!alliteration("lazy", "dog").unwrap());
}
