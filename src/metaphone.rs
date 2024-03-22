extern crate log;
extern crate pest;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Word;

#[derive(Debug, Clone, PartialEq)]
struct State {
    pos: usize,
    chars: Vec<char>,
    p: String,
    s: String,
}

impl State {
    fn new() -> State {
        State {
            pos: 0,
            chars: vec![],
            p: String::new(),
            s: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoubleMetaphone {
    pub primary: String,
    pub secondary: String,
}

/// Use Double Metaphone phonetic encoding to determine if two words rhyme.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// // Does rhyme
/// assert!(ttaw::metaphone::rhyme("far", "tar"));
/// assert!(ttaw::metaphone::rhyme("here", "near"));
///
/// // Does not rhyme
/// assert!(!ttaw::metaphone::rhyme("shopping", "cart"));
/// assert!(!ttaw::metaphone::rhyme("run", "uphill"));
/// ```
pub fn rhyme(a: &str, b: &str) -> bool {
    // sanity check, needing to sanity check seems fragile?
    if a.trim().is_empty() || b.trim().is_empty() {
        return false;
    }

    let a_phonetic = encoding(a);
    let b_phonetic = encoding(b);

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

/// Use Double Metaphone phonetic encoding to determine if two words alliterate.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
// // Does alliterate
/// assert!(ttaw::metaphone::alliteration("bouncing", "bears"));
/// assert!(ttaw::metaphone::alliteration("snappy", "snails"));
///
/// // Does not alliterate
/// assert!(!ttaw::metaphone::alliteration("brown", "fox"));
/// assert!(!ttaw::metaphone::alliteration("lazy", "dog"));
/// ```
pub fn alliteration(a: &str, b: &str) -> bool {
    if Word::parse(Rule::vowel_first, a.get(..1).unwrap_or_default()).is_ok() {
        return false;
    }

    if Word::parse(Rule::vowel_first, b.get(..1).unwrap_or_default()).is_ok() {
        return false;
    }

    let a_phonetic = encoding(a);
    let b_phonetic = encoding(b);

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

    if a_phonetic_head_primary == b_phonetic_head_primary
        || a_phonetic_head_primary == b_phonetic_head_secondary
        || a_phonetic_head_secondary == b_phonetic_head_primary
        || a_phonetic_head_secondary == b_phonetic_head_secondary
    {
        return true;
    }

    false
}

/// Double Metaphone phonetic encoding.
///
/// ```rust
/// extern crate ttaw;
/// use ttaw;
/// assert_eq!(ttaw::metaphone::encoding("Arnow").primary, "ARN");
/// assert_eq!(ttaw::metaphone::encoding("Arnow").secondary, "ARNF");
///
/// assert_eq!(ttaw::metaphone::encoding("detestable").primary, "TTSTPL");
/// assert_eq!(ttaw::metaphone::encoding("detestable").secondary, "TTSTPL");
/// ```
///
pub fn encoding(input: &str) -> DoubleMetaphone {
    let mut state = State::new();
    let word: String = input.to_uppercase() + "     ";

    state.chars = word.chars().collect::<Vec<char>>();

    if Word::parse(Rule::initial_exceptions, word.as_str()).is_ok() {
        state.pos += 1;
    }

    if let Some('X') = state.chars.first() {
        state.p += "S";
        state.s += "S";
        state.pos += 1
    }

    while let Some(c) = state.chars.get(state.pos) {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'À' | 'Ê' | 'É' => {
                vowel_case(&mut state);
            }

            'B' => {
                b_case(&mut state);
            }

            'Ç' => {
                c_cedilla_case(&mut state);
            }

            'C' => {
                c_case(&mut state);
            }

            'D' => {
                d_case(&mut state);
            }

            'F' => {
                f_case(&mut state);
            }

            'G' => {
                g_case(&mut state);
            }

            'H' => {
                h_case(&mut state);
            }

            'J' => {
                j_case(&mut state);
            }

            'K' => {
                k_case(&mut state);
            }

            'L' => {
                l_case(&mut state);
            }

            'M' => {
                m_case(&mut state);
            }

            'N' => {
                n_case(&mut state);
            }

            'Ñ' => {
                top_tilde_n_case(&mut state);
            }

            'P' => {
                p_case(&mut state);
            }

            'Q' => {
                q_case(&mut state);
            }

            'R' => {
                r_case(&mut state);
            }

            'S' => {
                s_case(&mut state);
            }

            'T' => {
                t_case(&mut state);
            }

            'V' => {
                v_case(&mut state);
            }

            'W' => {
                w_case(&mut state);
            }

            'X' => {
                x_case(&mut state);
            }

            'Z' => {
                z_case(&mut state);
            }

            _ => state.pos += 1,
        }
    }

    DoubleMetaphone {
        primary: state.p,
        secondary: state.s,
    }
}

fn get_char_as_string(chars: &[char], pos: usize) -> String {
    match chars.get(pos) {
        Some(c) => c.to_string(),
        None => String::new(),
    }
}

fn get_substring(chars: &[char], start: usize, end: usize) -> String {
    match chars.get(start..end) {
        Some(s) => s.iter().collect::<String>(),
        None => String::new(),
    }
}

fn germanic(chars: &[char]) -> bool {
    Word::parse(Rule::germanic, chars.iter().collect::<String>().as_str()).is_ok()
}

fn slavo_germanic(chars: &[char]) -> bool {
    Word::parse(
        Rule::slavo_germanic,
        chars.iter().collect::<String>().as_str(),
    )
    .is_ok()
}

fn vowel_case(State { pos, p, s, .. }: &mut State) {
    if *pos == 0 {
        *p += "A";
        *s += "A";
    }

    *pos += 1;
}

fn b_case(State { pos, chars, p, s }: &mut State) {
    *p += "P";
    *s += "P";

    if let Some('B') = chars.get(*pos + 1) {
        *pos += 1;
    }

    *pos += 1;
}

fn c_cedilla_case(State { pos, p, s, .. }: &mut State) {
    *p += "S";
    *s += "S";
    *pos += 1;
}

fn c_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(pos.wrapping_sub(1)) == Some(&'A')
        && chars.get(*pos + 1) == Some(&'H')
        && chars.get(*pos + 2) != Some(&'I')
        && Word::parse(
            Rule::vowels,
            get_char_as_string(chars, pos.wrapping_sub(3)).as_str(),
        )
        .is_err()
        && (chars.get(*pos + 2) != Some(&'E')
            || get_substring(chars, pos.wrapping_sub(2), *pos + 4) == "BACHER"
            || get_substring(chars, pos.wrapping_sub(2), *pos + 4) == "MACHER")
    {
        *p += "K";
        *s += "K";
        *pos += 2;

        return;
    }

    if *pos == 0 && get_substring(chars, 1, 6) == "AESAR" {
        *p += "S";
        *s += "S";
        *pos += 2;

        return;
    }

    if get_substring(chars, *pos + 1, *pos + 4) == "HIA" {
        *p += "K";
        *s += "K";
        *pos += 2;

        return;
    }

    if let Some('H') = chars.get(*pos + 1) {
        if *pos > 0 && chars.get(*pos + 2) == Some(&'A') && chars.get(*pos + 3) == Some(&'E') {
            *p += "K";
            *s += "X";
            *pos += 2;

            return;
        }

        if *pos == 0
            && Word::parse(
                Rule::initial_greek_ch,
                chars.iter().collect::<String>().as_str(),
            )
            .is_ok()
        {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        if germanic(chars)
            || Word::parse(
                Rule::greek_ch,
                get_substring(chars, pos.wrapping_sub(2), *pos + 4).as_str(),
            )
            .is_ok()
            || (get_char_as_string(chars, *pos + 2) == "T"
                || get_char_as_string(chars, *pos + 2) == "S")
            || ((*pos == 0
                || chars.get(pos.wrapping_sub(1)) == Some(&'A')
                || chars.get(pos.wrapping_sub(1)) == Some(&'E')
                || chars.get(pos.wrapping_sub(1)) == Some(&'O')
                || chars.get(pos.wrapping_sub(1)) == Some(&'U'))
                && Word::parse(Rule::ch_for_k, get_char_as_string(chars, *pos + 2).as_str())
                    .is_ok())
        {
            *p += "K";
            *s += "K";
        } else if *pos == 0 {
            *p += "X";
            *s += "X";
        } else if get_substring(chars, 0, 2) == "MC" {
            *p += "K";
            *s += "K";
        } else {
            *p += "X";
            *s += "K"
        }

        *pos += 2;
        return;
    }

    if chars.get(*pos + 1) == Some(&'Z') && get_substring(chars, pos.wrapping_sub(2), *pos) != "WI"
    {
        *p += "S";
        *s += "X";
        *pos += 2;

        return;
    }

    if get_substring(chars, *pos + 1, *pos + 4) == "CIA" {
        *p += "X";
        *s += "X";
        *pos += 3;

        return;
    }

    if chars.get(*pos + 1) == Some(&'C') && !(*pos == 1 && chars.first() == Some(&'M')) {
        if (chars.get(*pos + 2) == Some(&'I')
            || chars.get(*pos + 2) == Some(&'E')
            || chars.get(*pos + 2) == Some(&'H'))
            && get_substring(chars, *pos + 2, *pos + 4) != "HU"
        {
            if (*pos == 1 && chars.get(pos.wrapping_sub(1)) == Some(&'A'))
                || get_substring(chars, pos.wrapping_sub(1), *pos + 4) == "UCCEE"
                || get_substring(chars, pos.wrapping_sub(1), *pos + 4) == "UCCES"
            {
                *p += "KS";
                *s += "KS";
            } else {
                *p += "X";
                *s += "X";
            }

            *pos += 3;
        } else {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        return;
    }

    if Some(&'G') == chars.get(*pos + 1)
        || Some(&'K') == chars.get(*pos + 1)
        || Some(&'Q') == chars.get(*pos + 1)
    {
        *p += "K";
        *s += "K";

        *pos += 2;
        return;
    }

    if Some(&'I') == chars.get(*pos + 1)
        && (Some(&'E') == chars.get(*pos + 2) || Some(&'O') == chars.get(*pos + 2))
    {
        *p += "S";
        *s += "X";
        *pos += 2;
        return;
    }

    if Some(&'I') == chars.get(*pos + 1)
        || Some(&'E') == chars.get(*pos + 1)
        || Some(&'Y') == chars.get(*pos + 1)
    {
        *p += "S";
        *s += "S";
        *pos += 2;
        return;
    }

    *p += "K";
    *s += "K";

    if Some(&' ') == chars.get(*pos + 1)
        && (Some(&'C') == chars.get(*pos + 2)
            || Some(&'G') == chars.get(*pos + 2)
            || Some(&'Q') == chars.get(*pos + 2))
    {
        *pos += 3;
        return;
    }

    *pos += 1;
}

fn d_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'G') {
        if chars.get(*pos + 2) == Some(&'E')
            || chars.get(*pos + 2) == Some(&'I')
            || chars.get(*pos + 2) == Some(&'Y')
        {
            *p += "J";
            *s += "J";
            *pos += 3;
        } else {
            *p += "TK";
            *s += "TK";
            *pos += 2;
        }

        return;
    }

    if chars.get(*pos + 1) == Some(&'T') || chars.get(*pos + 1) == Some(&'D') {
        *p += "T";
        *s += "T";
        *pos += 2;

        return;
    }

    *p += "T";
    *s += "T";
    *pos += 1;
}

fn f_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'F') {
        *pos += 1;
    }

    *pos += 1;
    *p += "F";
    *s += "F";
}

fn g_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'H') {
        if *pos > 0
            && Word::parse(
                Rule::vowels,
                get_char_as_string(chars, pos.wrapping_sub(1)).as_str(),
            )
            .is_err()
        {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        if *pos == 0 {
            if chars.get(*pos + 2) == Some(&'I') {
                *p += "J";
                *s += "J";
            } else {
                *p += "K";
                *s += "K";
            }

            *pos += 2;

            return;
        }

        if (chars.get(pos.wrapping_sub(2)) == Some(&'B')
            || chars.get(pos.wrapping_sub(2)) == Some(&'H')
            || chars.get(pos.wrapping_sub(2)) == Some(&'D'))
            || (chars.get(pos.wrapping_sub(3)) == Some(&'B')
                || chars.get(pos.wrapping_sub(3)) == Some(&'H')
                || chars.get(pos.wrapping_sub(3)) == Some(&'D'))
            || (chars.get(pos.wrapping_sub(4)) == Some(&'B')
                || chars.get(pos.wrapping_sub(4)) == Some(&'H'))
        {
            *pos += 2;

            return;
        }

        if *pos > 2
            && chars.get(pos.wrapping_sub(1)) == Some(&'U')
            && Word::parse(
                Rule::g_for_f,
                get_char_as_string(chars, pos.wrapping_sub(3)).as_str(),
            )
            .is_ok()
        {
            *p += "F";
            *s += "F";
        } else if *pos > 0 && chars.get(pos.wrapping_sub(1)) != Some(&'I') {
            *p += "K";
            *s += "K";
        }

        *pos += 2;

        return;
    }

    if chars.get(*pos + 1) == Some(&'N') {
        if *pos == 1
            && Word::parse(Rule::vowels, get_char_as_string(chars, 0).as_str()).is_ok()
            && !slavo_germanic(chars)
        {
            *p += "KN";
            *s += "N";
        } else if get_substring(chars, *pos + 2, *pos + 4) != "EY"
            && chars.get(*pos + 1) != Some(&'Y')
            && !slavo_germanic(chars)
        {
            *p += "N";
            *s += "KN"
        } else {
            *p += "KN";
            *s += "KN"
        }

        *pos += 2;

        return;
    }

    if get_substring(chars, *pos + 1, *pos + 3) == "LI" && !slavo_germanic(chars) {
        *p += "KL";
        *s += "L";
        *pos += 2;

        return;
    }

    if *pos == 0
        && Word::parse(
            Rule::initial_g_or_for_k_or_j,
            get_substring(chars, 1, 3).as_str(),
        )
        .is_ok()
    {
        *p += "K";
        *s += "J";
        *pos += 2;

        return;
    }

    if get_substring(chars, *pos + 1, *pos + 3) == "ER"
        && chars.get(pos.wrapping_sub(1)) != Some(&'I')
        && chars.get(pos.wrapping_sub(1)) != Some(&'E')
        && Word::parse(
            Rule::initial_anger_exception,
            get_substring(chars, 0, 6).as_str(),
        )
        .is_err()
        || (chars.get(*pos + 1) == Some(&'Y')
            && Word::parse(
                Rule::g_for_k_or_j,
                get_char_as_string(chars, pos.wrapping_sub(1)).as_str(),
            )
            .is_err())
    {
        *p += "K";
        *s += "J";
        *pos += 2;

        return;
    }

    if chars.get(*pos + 1) == Some(&'E')
        || chars.get(*pos + 1) == Some(&'I')
        || chars.get(*pos + 1) == Some(&'Y')
        || ((chars.get(pos.wrapping_sub(1)) == Some(&'A')
            || chars.get(pos.wrapping_sub(1)) == Some(&'O'))
            && chars.get(*pos + 1) == Some(&'G')
            && chars.get(*pos + 2) == Some(&'I'))
    {
        if get_substring(chars, *pos + 1, *pos + 3) == "ET" || germanic(chars) {
            *p += "K";
            *s += "K";
        } else {
            *p += "J";

            if get_substring(chars, *pos + 1, *pos + 5) == "IER " {
                *s += "J";
            } else {
                *s += "K";
            }
        }

        *pos += 2;

        return;
    }

    if chars.get(*pos + 1) == Some(&'G') {
        *pos += 1;
    }

    *pos += 1;

    *p += "K";
    *s += "K";
}

fn h_case(State { pos, chars, p, s }: &mut State) {
    if Word::parse(Rule::vowels, get_char_as_string(chars, *pos + 1).as_str()).is_ok()
        && (*pos == 0
            || Word::parse(
                Rule::vowels,
                get_char_as_string(chars, pos.wrapping_sub(1)).as_str(),
            )
            .is_ok())
    {
        *p += "H";
        *s += "H";

        *pos += 1;
    }

    *pos += 1;
}

fn j_case(State { pos, chars, p, s }: &mut State) {
    if get_substring(chars, *pos, *pos + 4) == "JOSE" || get_substring(chars, 0, 4) == "SAN " {
        if get_substring(chars, 0, 4) == "SAN " || (*pos == 0 && chars.get(*pos + 4) == Some(&' '))
        {
            *p += "H";
            *s += "H";
        } else {
            *p += "J";
            *s += "H";
        }

        *pos += 1;

        return;
    }

    if *pos == 0 {
        *p += "J";
        *s += "A";
    } else if !slavo_germanic(chars)
        && (chars.get(*pos + 1) == Some(&'A') || chars.get(*pos + 1) == Some(&'O'))
        && Word::parse(
            Rule::vowels,
            get_char_as_string(chars, pos.wrapping_sub(1)).as_str(),
        )
        .is_ok()
    {
        *p += "J";
        *s += "H";
    } else if *pos == chars.len().wrapping_sub(6) {
        *p += "J";
    } else if chars.get(pos.wrapping_sub(1)) != Some(&'S')
        && chars.get(pos.wrapping_sub(1)) != Some(&'K')
        && chars.get(pos.wrapping_sub(1)) != Some(&'L')
        && Word::parse(
            Rule::j_for_j_exception,
            get_char_as_string(chars, *pos + 1).as_str(),
        )
        .is_err()
    {
        *p += "J";
        *s += "J";
    } else if chars.get(*pos + 1) == Some(&'J') {
        *pos += 1;
    }

    *pos += 1;
}

fn k_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'K') {
        *pos += 1;
    }

    *p += "K";
    *s += "K";
    *pos += 1;
}

fn l_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'L') {
        if *pos == chars.len().wrapping_sub(8)
            && ((chars.get(pos.wrapping_sub(1)) == Some(&'A') && chars.get(*pos + 2) == Some(&'E'))
                || (chars.get(pos.wrapping_sub(1)) == Some(&'I')
                    && (chars.get(*pos + 2) == Some(&'O') || chars.get(*pos + 2) == Some(&'A'))))
            || (chars.get(pos.wrapping_sub(1)) == Some(&'A')
                && chars.get(*pos + 2) == Some(&'E')
                && (chars.get(chars.len().wrapping_sub(6)) == Some(&'A')
                    || chars.get(chars.len().wrapping_sub(6)) == Some(&'O')
                    || Word::parse(
                        Rule::alle,
                        get_substring(
                            chars,
                            chars.len().wrapping_sub(7),
                            chars.len().wrapping_sub(5),
                        )
                        .as_str(),
                    )
                    .is_ok()))
        {
            *p += "L";
            *pos += 2;

            return;
        }

        *pos += 1;
    }

    *p += "L";
    *s += "L";
    *pos += 1;
}

fn m_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'M')
        || (chars.get(pos.wrapping_sub(1)) == Some(&'U')
            && chars.get(*pos + 1) == Some(&'B')
            && (*pos + 1 == chars.len().wrapping_sub(6)
                || get_substring(chars, *pos + 2, *pos + 4) == "ER"))
    {
        *pos += 1;
    }

    *pos += 1;
    *p += "M";
    *s += "M";
}

fn n_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'N') {
        *pos += 1;
    }

    *pos += 1;
    *p += "N";
    *s += "N";
}

fn top_tilde_n_case(State { pos, p, s, .. }: &mut State) {
    *pos += 1;
    *p += "N";
    *s += "N";
}

fn p_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'H') {
        *p += "F";
        *s += "F";
        *pos += 2;

        return;
    }

    if chars.get(*pos + 1) == Some(&'P') || chars.get(*pos + 1) == Some(&'B') {
        *pos += 1;
    }

    *pos += 1;

    *p += "P";
    *s += "P";
}

fn q_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'Q') {
        *pos += 1;
    }

    *pos += 1;
    *p += "K";
    *s += "K";
}

fn r_case(State { pos, chars, p, s }: &mut State) {
    if *pos == chars.len().wrapping_sub(6)
        && !slavo_germanic(chars)
        && chars.get(pos.wrapping_sub(1)) == Some(&'E')
        && chars.get(pos.wrapping_sub(2)) == Some(&'I')
        && chars.get(pos.wrapping_sub(4)) != Some(&'M')
        && (chars.get(pos.wrapping_sub(3)) != Some(&'E')
            && chars.get(pos.wrapping_sub(3)) != Some(&'A'))
    {
        *s += "R";
    } else {
        *p += "R";
        *s += "R";
    }

    if chars.get(*pos + 1) == Some(&'R') {
        *pos += 1;
    }

    *pos += 1;
}

fn s_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'L')
        && (chars.get(pos.wrapping_sub(1)) == Some(&'I')
            || chars.get(pos.wrapping_sub(1)) == Some(&'Y'))
    {
        *pos += 1;

        return;
    }

    if *pos == 0 && get_substring(chars, 1, 5) == "UGAR" {
        *p += "X";
        *s += "S";
        *pos += 1;

        return;
    }

    if chars.get(*pos + 1) == Some(&'H') {
        if Word::parse(
            Rule::h_for_s,
            get_substring(chars, *pos + 1, *pos + 5).as_str(),
        )
        .is_ok()
        {
            *p += "S";
            *s += "S";
        } else {
            *p += "X";
            *s += "X";
        }

        *pos += 2;
        return;
    }

    if chars.get(*pos + 1) == Some(&'I')
        && (chars.get(*pos + 2) == Some(&'O') || chars.get(*pos + 2) == Some(&'A'))
    {
        if slavo_germanic(chars) {
            *p += "S";
            *s += "S";
        } else {
            *p += "S";
            *s += "X";
        }

        *pos += 3;

        return;
    }

    if chars.get(*pos + 1) == Some(&'Z')
        || (*pos == 0
            && (chars.get(*pos + 1) == Some(&'L')
                || chars.get(*pos + 1) == Some(&'M')
                || chars.get(*pos + 1) == Some(&'N')
                || chars.get(*pos + 1) == Some(&'W')))
    {
        *p += "S";
        *s += "X";

        if chars.get(*pos + 1) == Some(&'Z') {
            *pos += 1;
        }

        *pos += 1;

        return;
    }

    if chars.get(*pos + 1) == Some(&'C') {
        if chars.get(*pos + 2) == Some(&'H') {
            if Word::parse(
                Rule::dutch_sch,
                get_substring(chars, *pos + 3, *pos + 5).as_str(),
            )
            .is_ok()
            {
                if get_substring(chars, *pos + 3, *pos + 5) == "ER"
                    || get_substring(chars, *pos + 3, *pos + 5) == "EN"
                {
                    *p += "X";
                    *s += "SK"
                } else {
                    *p += "SK";
                    *s += "SK"
                }

                *pos += 3;

                return;
            }

            if *pos == 0
                && Word::parse(Rule::vowels, get_char_as_string(chars, 3).as_str()).is_err()
                && chars.get(3) != Some(&'W')
            {
                *p += "X";
                *s += "S";
            } else {
                *p += "X";
                *s += "X";
            }

            *pos += 3;

            return;
        }

        if chars.get(*pos + 2) == Some(&'I')
            || chars.get(*pos + 2) == Some(&'E')
            || chars.get(*pos + 2) == Some(&'Y')
        {
            *p += "S";
            *s += "S";
            *pos += 3;
            return;
        }

        *p += "SK";
        *s += "SK";
        *pos += 3;

        return;
    }

    if *pos == chars.len().wrapping_sub(6)
        && (get_substring(chars, pos.wrapping_sub(2), *pos) == "AI"
            || get_substring(chars, pos.wrapping_sub(2), *pos) == "OI")
    {
        *s += "S";
    } else {
        *p += "S";
        *s += "S";
    }

    if chars.get(*pos + 1) == Some(&'S') {
        *pos += 1;
    }

    *pos += 1;
}

fn t_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'I')
        && chars.get(*pos + 2) == Some(&'O')
        && chars.get(*pos + 3) == Some(&'N')
    {
        *p += "X";
        *s += "X";
        *pos += 3;

        return;
    }

    if (chars.get(*pos + 1) == Some(&'I') && chars.get(*pos + 2) == Some(&'A'))
        || (chars.get(*pos + 1) == Some(&'C') && chars.get(*pos + 2) == Some(&'H'))
    {
        *p += "X";
        *s += "X";
        *pos += 3;

        return;
    }

    if chars.get(*pos + 1) == Some(&'H')
        || (chars.get(*pos + 1) == Some(&'T') && chars.get(*pos + 2) == Some(&'H'))
    {
        if germanic(chars)
            || ((chars.get(*pos + 2) == Some(&'O') || chars.get(*pos + 2) == Some(&'A'))
                && chars.get(*pos + 3) == Some(&'M'))
        {
            *p += "T";
            *s += "T";
        } else {
            *p += "0";
            *s += "T";
        }

        *pos += 2;

        return;
    }

    if chars.get(*pos + 1) == Some(&'T') || chars.get(*pos + 1) == Some(&'D') {
        *pos += 1;
    }

    *pos += 1;
    *p += "T";
    *s += "T";
}

fn v_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'V') {
        *pos += 1;
    }

    *p += "F";
    *s += "F";
    *pos += 1;
}

fn w_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'R') {
        *p += "R";
        *s += "R";
        *pos += 2;

        return;
    }

    if *pos == 0 {
        if Word::parse(Rule::vowels, get_char_as_string(chars, *pos + 1).as_str()).is_ok() {
            *p += "A";
            *s += "F";
        } else if chars.get(*pos + 1) == Some(&'H') {
            *p += "A";
            *s += "A";
        }
    }

    if ((chars.get(pos.wrapping_sub(1)) == Some(&'E')
        || chars.get(pos.wrapping_sub(1)) == Some(&'O'))
        && chars.get(*pos + 1) == Some(&'S')
        && chars.get(*pos + 2) == Some(&'K')
        && (chars.get(*pos + 3) == Some(&'I') || chars.get(*pos + 3) == Some(&'Y')))
        || get_substring(chars, 0, 3) == "SCH"
        || (*pos == chars.len().wrapping_sub(6)
            && Word::parse(
                Rule::vowels,
                get_char_as_string(chars, pos.wrapping_sub(1)).as_str(),
            )
            .is_ok())
    {
        *s += "F";
        *pos += 1;

        return;
    }

    if chars.get(*pos + 1) == Some(&'I')
        && (chars.get(*pos + 2) == Some(&'C') || chars.get(*pos + 2) == Some(&'T'))
        && chars.get(*pos + 3) == Some(&'Z')
    {
        *p += "TS";
        *s += "FX";
        *pos += 4;

        return;
    }

    *pos += 1;
}

fn x_case(State { pos, chars, p, s }: &mut State) {
    if !(*pos == chars.len().wrapping_sub(6)
        && (chars.get(pos.wrapping_sub(1)) == Some(&'U')
            && (chars.get(pos.wrapping_sub(2)) == Some(&'A')
                || chars.get(pos.wrapping_sub(2)) == Some(&'O'))))
    {
        *p += "KS";
        *s += "KS"
    }

    if chars.get(*pos + 1) == Some(&'C') || chars.get(*pos + 1) == Some(&'X') {
        *pos += 1;
    }

    *pos += 1;
}

fn z_case(State { pos, chars, p, s }: &mut State) {
    if chars.get(*pos + 1) == Some(&'H') {
        *p += "J";
        *s += "J";
        *pos += 2;

        return;
    } else if (chars.get(*pos + 1) == Some(&'Z')
        && (chars.get(*pos + 2) == Some(&'A')
            || chars.get(*pos + 2) == Some(&'I')
            || chars.get(*pos + 2) == Some(&'O')))
        || (slavo_germanic(chars) && *pos > 0 && chars.get(pos.wrapping_sub(1)) != Some(&'T'))
    {
        *p += "S";
        *s += "TS"
    } else {
        *p += "S";
        *s += "S";
    }

    if chars.get(*pos + 1) == Some(&'Z') {
        *pos += 1;
    }

    *pos += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slavo_germanic() {
        let mut word = "apple".to_uppercase();
        assert!(Word::parse(Rule::slavo_germanic, word.as_str()).is_err());
        word = "witzig".to_uppercase();
        assert!(Word::parse(Rule::slavo_germanic, word.as_str()).is_ok());
        word = "watt".to_uppercase();
        assert!(Word::parse(Rule::slavo_germanic, word.as_str()).is_ok());
        word = "kilometer".to_uppercase();
        assert!(Word::parse(Rule::slavo_germanic, word.as_str()).is_ok());
        word = "eczema".to_uppercase();
        assert!(Word::parse(Rule::slavo_germanic, word.as_str()).is_ok());
    }

    #[test]
    fn germanic() {
        let mut word = "tomato".to_uppercase();
        assert!(Word::parse(Rule::germanic, word.as_str()).is_err());
        word = "vanity".to_uppercase();
        assert!(Word::parse(Rule::germanic, word.as_str()).is_ok());
        word = "vondur".to_uppercase();
        assert!(Word::parse(Rule::germanic, word.as_str()).is_ok());
        word = "schema".to_uppercase();
        assert!(Word::parse(Rule::germanic, word.as_str()).is_ok());
    }

    #[test]
    fn initial_exceptions() {
        let mut word = "spruce".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_err());
        word = "gnome".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_ok());
        word = "knight".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_ok());
        word = "pneumonic".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_ok());
        word = "wrangle".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_ok());
        word = "pseudo".to_uppercase();
        assert!(Word::parse(Rule::initial_exceptions, word.as_str()).is_ok());
    }

    #[test]
    fn initial_greek_ch() {
        let mut word = "tulip".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());

        word = "pliant".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "chiaroscuro".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());

        word = "seem".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "chemistry".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());

        word = "organ".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "oregon".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "chores".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "chorus".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());

        word = "ymca".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "chymera".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());

        word = "arachnid".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "character".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());

        word = "aristotle".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_err());
        word = "charisma".to_uppercase();
        assert!(Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok());
    }

    #[test]
    fn vowels() {
        let mut letter = "X";
        assert!(Word::parse(Rule::vowels, letter).is_err());
        letter = "À";
        assert!(Word::parse(Rule::vowels, letter).is_err());
        letter = "A";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
        letter = "E";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
        letter = "I";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
        letter = "O";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
        letter = "U";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
        letter = "Y";
        assert!(Word::parse(Rule::vowels, letter).is_ok());
    }

    #[test]
    fn greek_ch() {
        let mut word = "cucumber".to_uppercase();
        assert!(Word::parse(Rule::greek_ch, word.as_str()).is_err());
        word = "arch".to_uppercase();
        assert!(Word::parse(Rule::greek_ch, word.as_str()).is_err());
        word = "architect".to_uppercase();
        assert!(Word::parse(Rule::greek_ch, word.as_str()).is_ok());
        word = "orchestra".to_uppercase();
        assert!(Word::parse(Rule::greek_ch, word.as_str()).is_ok());
        word = "orchid".to_uppercase();
        assert!(Word::parse(Rule::greek_ch, word.as_str()).is_ok());
    }
}
