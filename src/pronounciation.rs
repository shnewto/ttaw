extern crate pest;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Word;

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

#[derive(Debug, Clone, PartialEq)]
struct Metaphone {
    pos: usize,
    chars: Vec<char>,
    p: String,
    s: String,
}

impl Metaphone {
    pub fn new() -> Metaphone {
        Metaphone {
            pos: 0,
            chars: vec![],
            p: String::new(),
            s: String::new(),
        }
    }
}

pub fn double_metaphone(input: &str) -> Result<Vec<String>, ()> {
    let mut metaphone = Metaphone::new();
    let word: String = input.to_uppercase();

    metaphone.chars = word.chars().collect::<Vec<char>>();

    if Word::parse(Rule::initial_exceptions, word.as_str()).is_ok() {
        metaphone.pos += 1;
    } else if let Some('X') = metaphone.chars.first() {
        metaphone.pos += 1
    }

    while let Some(c) = metaphone.chars.get(metaphone.pos) {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'À' | 'Ê' | 'É' => {
                vowel_case(&mut metaphone);
            }

            'B' => {
                b_case(&mut metaphone);
            }

            'C' => {
                c_case(&mut metaphone);
            }

            'Ç' => {
                c_cedilla_case(&mut metaphone);
            }

            'D' => {
                d_case(&mut metaphone);
            }

            'F' => {
                f_case(&mut metaphone);
            }

            'G' => {
                g_case(&mut metaphone);
            }

            'H' => {
                h_case(&mut metaphone);
            }

            'J' => {
                j_case(&mut metaphone);
            }

            'K' => {
                k_case(&mut metaphone);
            }

            'L' => {
                l_case(&mut metaphone);
            }

            'M' => {
                m_case(&mut metaphone);
            }

            'N' => {
                n_case(&mut metaphone);
            }

            'P' => {
                p_case(&mut metaphone);
            }

            'Q' => {
                q_case(&mut metaphone);
            }

            'R' => {
                r_case(&mut metaphone);
            }

            'S' => {
                s_case(&mut metaphone);
            }

            'T' => {
                t_case(&mut metaphone);
            }

            'V' => {
                v_case(&mut metaphone);
            }

            'W' => {
                w_case(&mut metaphone);
            }

            'X' => {
                x_case(&mut metaphone);
            }

            'Z' => {
                z_case(&mut metaphone);
            }

            _ => metaphone.pos += 1,
        }
    }

    Ok(vec![metaphone.p, metaphone.s])
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

fn vowel_case(Metaphone { pos, p, s, .. }: &mut Metaphone) {
    // fn vowel_case( pos: &mut usize, _chars: &[char], p: &mut String, s: &mut String) {
    if *pos == 0 {
        *p += "A";
        *s += "A";
    }

    *pos += 1;
}

fn b_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    *p += "P";
    *s += "P";

    if let Some('B') = chars.get(*pos + 1) {
        *pos += 1;
    }

    *pos += 1;
}

fn c_cedilla_case(Metaphone { pos, p, s, .. }: &mut Metaphone) {
    *p += "S";
    *s += "S";
    *pos += 1;
}

fn c_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(pos.wrapping_sub(1)) == Some(&'A')
        && chars.get(*pos + 1) == Some(&'H')
        && chars.get(*pos + 2) != Some(&'I')
    {
        let sub_value: String = get_substring(&chars, pos.wrapping_sub(2), *pos + 4);
        if sub_value.as_str() == "BACHER" || sub_value.as_str() == "MACHER" {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        if "AESAR" == get_substring(&chars, 1, 6) {
            *p += "S";
            *s += "S";
            *pos += 2;

            return;
        }

        if "HIA" == get_substring(&chars, *pos + 1, *pos + 4) {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        if let Some('H') = chars.get(*pos + 1) {
            if "AE" == get_substring(&chars, *pos + 2, *pos + 4) {
                *p += "K";
                *s += "X";
                *pos += 2;

                return;
            }

            if Word::parse(
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

            let mut pos_plus_2 = String::new();

            if let Some(pp2) = chars.get(*pos + 2) {
                pos_plus_2 = pp2.to_string();
            }

            if germanic(&chars)
                || pos_plus_2 == "T"
                || pos_plus_2 == "S"
                || ((*pos == 0
                    || chars.get(pos.wrapping_sub(1)) == Some(&'A')
                    || chars.get(pos.wrapping_sub(1)) == Some(&'E')
                    || chars.get(pos.wrapping_sub(1)) == Some(&'I')
                    || chars.get(pos.wrapping_sub(1)) == Some(&'O')
                    || chars.get(pos.wrapping_sub(1)) == Some(&'U'))
                    && Word::parse(Rule::ch_for_k, pos_plus_2.as_str()).is_ok())
                || chars.get(..2) == Some(&['M', 'C'])
            {
                *p += "K";
                *s += "K";
            } else if *pos == 0 {
                *p += "X";
                *s += "X";
            } else {
                *p += "X";
                *s += "K"
            }

            *pos += 2;
            return;
        }

        if chars.get(*pos + 1) == Some(&'Z')
            && chars.get(pos.wrapping_sub(2)..*pos) == Some(&['W', 'I'])
        {
            *p += "S";
            *s += "X";
            *pos += 2;

            return;
        }

        if chars.get(*pos + 1..*pos + 4) == Some(&['C', 'I', 'A']) {
            *p += "X";
            *s += "X";
            *pos += 3;

            return;
        }

        if chars.get(*pos + 1) == Some(&'C') && !(*pos == 1 && chars.get(0) == Some(&'M')) {
            if (chars.get(*pos + 2) == Some(&'I')
                || chars.get(*pos + 2) == Some(&'E')
                || chars.get(*pos + 2) == Some(&'H'))
                && chars.get(*pos + 2..*pos + 4) != Some(&['H', 'U'])
            {
                let val = get_substring(&chars, pos.wrapping_sub(1), *pos + 4);

                if (*pos == 1 && chars.get(pos.wrapping_sub(1)) == Some(&'M'))
                    || val == "UCCEE"
                    || val == "UCCES"
                {
                    *p += "KS";
                    *s += "KS";
                } else {
                    *p += "X";
                    *s += "X";
                }
            } else {
                *p += "K";
                *s += "K";
                *pos += 2;
                return;
            }

            *pos += 3;
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

        if chars.get(*pos + 1) == Some(&'K')
            || chars.get(*pos + 1) == Some(&'Q')
            || (chars.get(*pos + 1) == Some(&'C')
                && chars.get(*pos + 2) != Some(&'E')
                && chars.get(*pos + 2) != Some(&'I'))
        {
            *pos += 1;
        }

        *pos += 1;
    }
}

fn d_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
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

fn f_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'F') {
        *pos += 1;
    }

    *pos += 1;
    *p += "F";
    *s += "F";
}

fn g_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'H') {
        let prev = get_char_as_string(&chars, pos.wrapping_sub(1));

        if *pos > 0 && Word::parse(Rule::vowels, prev.as_str()).is_err() {
            *p += "K";
            *s += "K";
            *pos += 2;

            return;
        }

        // Such as `Ghislane`, `Ghiradelli`.
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
        let x = chars.get(pos.wrapping_sub(2));
        let y = chars.get(pos.wrapping_sub(3));
        let z = chars.get(pos.wrapping_sub(4));

        if x == Some(&'B')
            || x == Some(&'H')
            || x == Some(&'D')
            || (y == Some(&'B') || y == Some(&'H') || y == Some(&'D'))
            || (z == Some(&'B') || z == Some(&'H'))
        {
            *pos += 2;

            return;
        }
        let char_as_string = get_char_as_string(&chars, pos.wrapping_sub(3));
        if *pos > 2
            && chars.get(pos.wrapping_sub(1)) == Some(&'U')
            && Word::parse(Rule::g_for_f, char_as_string.as_str()).is_ok()
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
            && Word::parse(Rule::vowels, get_char_as_string(&chars, 0).as_str()).is_ok()
            && !slavo_germanic(&chars)
        {
            *p += "KN";
            *s += "N";
        // Not like `Cagney`.
        } else if get_substring(&chars, *pos + 2, *pos + 4) != "EY"
            && chars.get(*pos + 1) != Some(&'Y')
            && !slavo_germanic(&chars)
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

    // Such as `Tagliaro`.
    if get_substring(&chars, *pos + 1, *pos + 3) == "LI" && !slavo_germanic(&chars) {
        *p += "KL";
        *s += "L";
        *pos += 2;

        return;
    }

    // -ges-, -gep-, -gel- at beginning.
    if *pos == 0
        && Word::parse(
            Rule::initial_g_or_for_k_or_j,
            get_substring(&chars, 1, 3).as_str(),
        )
        .is_ok()
    {
        *p += "K";
        *s += "J";
        *pos += 2;

        return;
    }

    // -ger-, -gy-.
    if get_substring(&chars, *pos + 1, *pos + 3) == "ER"
        && chars.get(pos.wrapping_sub(1)) != Some(&'I')
        && chars.get(pos.wrapping_sub(1)) != Some(&'E')
        && Word::parse(
            Rule::initial_anger_exception,
            get_substring(&chars, *pos + 1, *pos + 3).as_str(),
        )
        .is_ok()
        && Word::parse(
            Rule::initial_anger_exception,
            get_substring(&chars, 0, 6).as_str(),
        )
        .is_ok()
        || (chars.get(*pos + 1) == Some(&'Y')
            && Word::parse(
                Rule::initial_g_or_for_k_or_j,
                get_char_as_string(&chars, 1).as_str(),
            )
            .is_err())
    {
        *p += "K";
        *s += "J";
        *pos += 2;

        return;
    }

    // Italian such as `biaggi`.
    if chars.get(*pos + 1) == Some(&'E')
        || chars.get(*pos + 1) == Some(&'I')
        || chars.get(*pos + 1) == Some(&'Y')
        || ((chars.get(pos.wrapping_sub(1)) == Some(&'A')
            || chars.get(pos.wrapping_sub(1)) == Some(&'O'))
            && chars.get(*pos + 1) == Some(&'G')
            && chars.get(*pos + 2) == Some(&'I'))
    {
        if get_substring(&chars, *pos + 1, *pos + 3) == "ET" || germanic(&chars) {
            *p += "K";
            *s += "K";
        } else {
            *p += "J";

            // Always soft if French ending.
            if get_substring(&chars, *pos + 1, *pos + 5) == "IER " {
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

fn h_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    // Only keep if first & before vowel or btw. 2 vowels.
    if Word::parse(Rule::vowels, get_char_as_string(&chars, *pos + 1).as_str()).is_ok()
        && (*pos == 0
            || Word::parse(
                Rule::vowels,
                get_char_as_string(&chars, pos.wrapping_sub(1)).as_str(),
            )
            .is_ok())
    {
        *p += "H";
        *s += "H";

        *pos += 1;
    }

    *pos += 1;
}

fn j_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if get_substring(&chars, *pos, *pos + 4) == "JOSE" || get_substring(&chars, 0, 4) == "SAN " {
        if get_substring(&chars, *pos, *pos + 4) == "SAN "
            || (*pos == 0 && chars.get(*pos + 4) == Some(&' '))
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
    } else if !slavo_germanic(&chars)
        && (chars.get(*pos + 1) == Some(&'A') || chars.get(*pos + 1) == Some(&'O'))
        && Word::parse(
            Rule::vowels,
            get_char_as_string(&chars, pos.wrapping_sub(1)).as_str(),
        )
        .is_ok()
    {
        *p += "J";
        *s += "H";
    } else if *pos == chars.len().wrapping_sub(1) {
        *p += "J";
    } else if chars.get(pos.wrapping_sub(1)) != Some(&'S')
        && chars.get(pos.wrapping_sub(1)) != Some(&'K')
        && chars.get(pos.wrapping_sub(1)) != Some(&'L')
        && Word::parse(
            Rule::j_for_j_exception,
            get_char_as_string(&chars, *pos + 1).as_str(),
        )
        .is_err()
    {
        *p += "J";
        *s += "J";
    // It could happen.
    } else if chars.get(*pos + 1) == Some(&'J') {
        *pos += 1;
    }

    *pos += 1;
}

fn k_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'K') {
        *pos += 1;
    }

    *p += "K";
    *s += "K";
    *pos += 1;
}

fn l_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'L') {
        // Spanish such as `cabrillo`, `gallegos`.
        if *pos == chars.len().wrapping_sub(3)
            && ((chars.get(pos.wrapping_sub(1)) == Some(&'A') && chars.get(*pos + 2) == Some(&'E'))
                || (chars.get(pos.wrapping_sub(1)) == Some(&'I')
                    && (chars.get(*pos + 2) == Some(&'O') || chars.get(*pos + 2) == Some(&'A'))))
            || (chars.get(pos.wrapping_sub(1)) == Some(&'A')
                && chars.get(*pos + 2) == Some(&'E')
                && (chars.get(chars.len().wrapping_sub(1)) == Some(&'A')
                    || chars.get(chars.len().wrapping_sub(1)) == Some(&'O')
                    || Word::parse(
                        Rule::alle,
                        get_substring(
                            &chars,
                            chars.len().wrapping_sub(2),
                            chars.len().wrapping_sub(1),
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

fn m_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'M') ||
          // Such as `dumb`, `thumb`.
          (chars.get(pos.wrapping_sub(1))  ==Some(&'U') &&
            chars.get(*pos + 1) == Some(&'B') &&
            (*pos + 1 == chars.len().wrapping_sub(1)||  get_substring(&chars, *pos + 2, *pos + 4) == "ER"))
    {
        *pos += 1;
    }

    *pos += 1;
    *p += "M";
    *s += "M";
}

fn n_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'N') {
        *pos += 1;
    }

    *pos += 1;
    *p += "N";
    *s += "N";
}

fn p_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'H') {
        *p += "F";
        *s += "F";
        *pos += 2;

        return;
    }

    // Also account for `campbell` and `raspberry`.
    let subvalue = chars.get(*pos + 1);

    if subvalue == Some(&'P') || subvalue == Some(&'B') {
        *pos += 1;
    }

    *pos += 1;

    *p += "P";
    *s += "P";
}

fn q_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'Q') {
        *pos += 1;
    }

    *pos += 1;
    *p += "K";
    *s += "K";
}

fn r_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    // French such as `Rogier`, but exclude `Hochmeier`.
    if *pos == chars.len().wrapping_sub(1)
        && !slavo_germanic(&chars)
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

fn s_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    // Special cases `island`, `isle`, `carlisle`, `carlysle`.
    if chars.get(*pos + 1) == Some(&'L')
        && (chars.get(pos.wrapping_sub(1)) == Some(&'I')
            || chars.get(pos.wrapping_sub(1)) == Some(&'Y'))
    {
        *pos += 1;

        return;
    }

    // Special case `sugar-`.
    if *pos == 0 && get_substring(&chars, 1, 5) == "UGAR" {
        *p += "X";
        *s += "S";
        *pos += 1;

        return;
    }

    if chars.get(*pos + 1) == Some(&'H') {
        // Germanic.
        if Word::parse(
            Rule::h_for_s,
            get_substring(&chars, *pos + 1, *pos + 5).as_str(),
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
        if slavo_germanic(&chars) {
            *p += "S";
            *s += "S";
        } else {
            *p += "S";
            *s += "X";
        }

        *pos += 3;

        return;
    }

    // German & Anglicization's, such as `Smith` match `Schmidt`, `snider`
    // match `Schneider`. Also, -sz- in slavic language although in
    // hungarian it is pronounced `s`.
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
        // Schlesinger's rule.
        if chars.get(*pos + 2) == Some(&'H') {
            let subvalue = get_substring(&chars, *pos + 3, *pos + 5);
            if Word::parse(Rule::dutch_sch, subvalue.as_str()).is_ok() {
                // Such as `schermerhorn`, `schenker`.
                if subvalue == "ER" || subvalue == "EN" {
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
                && Word::parse(Rule::vowels, get_char_as_string(&chars, 3).as_str()).is_err()
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

    let subvalue = get_substring(&chars, pos.wrapping_sub(2), *pos);

    // French such as `resnais`, `artois`.
    if *pos == chars.len().wrapping_sub(1) && (subvalue == "AI" || subvalue == "OI") {
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

fn t_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
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
        // Special case `Thomas`, `Thames` or Germanic.
        if germanic(&chars)
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

fn v_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if chars.get(*pos + 1) == Some(&'V') {
        *pos += 1;
    }

    *p += "F";
    *s += "F";
    *pos += 1;
}

fn w_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    // Can also be in middle of word (as already taken care of for initial).
    if chars.get(*pos + 1) == Some(&'R') {
        *p += "R";
        *s += "R";
        *pos += 2;

        return;
    }

    if *pos == 0 {
        // `Wasserman` should match `Vasserman`.
        if Word::parse(Rule::vowels, get_char_as_string(&chars, *pos + 1).as_str()).is_ok() {
            *p += "A";
            *s += "F";
        } else if chars.get(*pos + 1) == Some(&'H') {
            // Need `Uomo` to match `Womo`.
            *p += "A";
            *s += "A";
        }
    }

    // `Arnow` should match `Arnoff`.
    if ((chars.get(pos.wrapping_sub(1))  ==Some(&'E') || chars.get(pos.wrapping_sub(1))  == Some(&'O')) &&
            chars.get(*pos + 1) == Some(&'S') &&
            chars.get(*pos + 2) == Some(&'K') &&
            (chars.get(*pos + 3) == Some(&'I') || chars.get(*pos + 3) == Some(&'Y'))) ||
          // Maybe a bug? Shouldn't this be general Germanic?
          get_substring(&chars, 0, 3) == "SCH" ||
          (*pos == chars.len().wrapping_sub(1) && Word::parse(Rule::vowels, get_char_as_string(&chars, pos.wrapping_sub(1)).as_str()).is_ok())
    {
        *s += "F";
        *pos += 1;

        return;
    }

    // Polish such as `Filipowicz`.
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

fn x_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    if !(*pos == chars.len().wrapping_sub(1)
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

fn z_case(Metaphone { pos, chars, p, s }: &mut Metaphone) {
    // Chinese pinyin such as `Zhao`.
    if chars.get(*pos + 1) == Some(&'H') {
        *p += "J";
        *s += "J";
        *pos += 2;

        return;
    } else if (chars.get(*pos + 1) == Some(&'Z')
        && (chars.get(*pos + 2) == Some(&'A')
            || chars.get(*pos + 2) == Some(&'I')
            || chars.get(*pos + 2) == Some(&'O')))
        || (slavo_germanic(&chars) && *pos > 0 && chars.get(pos.wrapping_sub(1)) != Some(&'T'))
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
