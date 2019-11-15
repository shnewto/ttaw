extern crate ttaw;

use ttaw::cmu::{alliteration, rhyme};

use tempfile;

#[test]
fn rhymes_with_spaces() {
    assert!(rhyme("far  ", "tar").unwrap());
    assert!(rhyme(" far", "tar").unwrap());
    assert!(rhyme("far", " tar").unwrap());
    assert!(rhyme("far", "tar  ").unwrap());
}

#[test]
fn rhymes_with_caps() {
    assert!(rhyme("Far", "tar").unwrap());
    assert!(rhyme("far", "Tar").unwrap());
    assert!(rhyme("fAr", "taR").unwrap());
    assert!(rhyme("far", "tAr").unwrap());
}

#[test]
fn perfect_single() {
    assert!(rhyme("far", "tar").unwrap());
    assert!(rhyme("a", "say").unwrap());
    assert!(rhyme("hissed", "mist").unwrap());
    assert!(rhyme("tryst", "wrist").unwrap());
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
    assert!(!rhyme("cleaver", "silver").unwrap());
    assert!(!rhyme("pitter", "patter").unwrap());
    assert!(!rhyme("bottle", "fiddle").unwrap());
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
