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

pub fn double_metaphone(input: &str) -> Result<Vec<String>, ()> {
    let word: String = input.to_uppercase();
    let mut primary = String::new();
    let mut secondary = String::new();

    let characters = word.chars().collect::<Vec<char>>();
    let mut pos: usize = 0;

    if Word::parse(Rule::initial_exceptions, word.as_str()).is_ok() {
        pos += 1;
    } else if let Some('X') = characters.first() {
        pos += 1
    }

    'a: while let Some(c) = characters.get(pos) {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'À' | 'Ê' | 'É' => {
                if pos == 0 {
                    primary += "A";
                    secondary += "A";
                }

                pos += 1;
            }

            'B' => {
                primary += "P";
                secondary += "P";

                if let Some('B') = characters.get(pos + 1) {
                    pos += 1;
                }

                pos += 1;
            }

            'Ç' => {
                primary += "S";
                secondary += "S";
                pos += 1;
            }

            'C' => {
                c_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'D' => {
                d_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'F' => {
                if characters.get(pos + 1) == Some(&'F') {
                    pos += 1;
                }

                pos += 1;
                primary += "F";
                secondary += "F";

                continue 'a;
            }
            'G' => {
                g_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'H' => {
                // Only keep if first & before vowel or btw. 2 vowels.
                if Word::parse(
                    Rule::vowels,
                    get_char_as_string(&characters, pos + 1).as_str(),
                )
                .is_ok()
                    && (pos == 0
                        || Word::parse(
                            Rule::vowels,
                            get_char_as_string(&characters, pos.wrapping_sub(1)).as_str(),
                        )
                        .is_ok())
                {
                    primary += "H";
                    secondary += "H";

                    pos += 1;
                }

                pos += 1;

                continue 'a;
            }
            'J' => {
                j_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'K' => {
                if characters.get(pos + 1) == Some(&'K') {
                    pos += 1;
                }

                primary += "K";
                secondary += "K";
                pos += 1;

                continue 'a;
            }
            'L' => {
                if characters.get(pos + 1) == Some(&'L') {
                    // Spanish such as `cabrillo`, `gallegos`.
                    if pos == characters.len() - 3
                        && ((characters.get(pos.wrapping_sub(1)) == Some(&'A')
                            && characters.get(pos + 2) == Some(&'E'))
                            || (characters.get(pos.wrapping_sub(1)) == Some(&'I')
                                && (characters.get(pos + 2) == Some(&'O')
                                    || characters.get(pos + 2) == Some(&'A'))))
                        || (characters.get(pos.wrapping_sub(1)) == Some(&'A')
                            && characters.get(pos + 2) == Some(&'E')
                            && (characters.get(characters.len()) == Some(&'A')
                                || characters.get(characters.len()) == Some(&'O')
                                || Word::parse(
                                    Rule::alle,
                                    get_substring(
                                        &characters,
                                        characters.len().wrapping_sub(1),
                                        characters.len(),
                                    )
                                    .as_str(),
                                )
                                .is_ok()))
                    {
                        primary += "L";
                        pos += 2;

                        continue 'a;
                    }

                    pos += 1;
                }

                primary += "L";
                secondary += "L";
                pos += 1;

                continue 'a;
            }

            'M' => {
                if characters.get(pos + 1) == Some(&'M') ||
          // Such as `dumb`, `thumb`.
          (characters.get(pos.wrapping_sub(1))  ==Some(&'U') &&
            characters.get(pos + 1) == Some(&'B') &&
            (pos + 1 == characters.len() ||  get_substring(&characters, pos + 2, pos + 4) == "ER"))
                {
                    pos += 1;
                }

                pos += 1;
                primary += "M";
                secondary += "M";

                continue 'a;
            }
            'N' => {
                if characters.get(pos + 1) == Some(&'N') {
                    pos += 1;
                }

                pos += 1;
                primary += "N";
                secondary += "N";

                continue 'a;
            }
            'P' => {
                if characters.get(pos + 1) == Some(&'H') {
                    primary += "F";
                    secondary += "F";
                    pos += 2;

                    continue 'a;
                }

                // Also account for `campbell` and `raspberry`.
                let subvalue = characters.get(pos + 1);

                if subvalue == Some(&'P') || subvalue == Some(&'B') {
                    pos += 1;
                }

                pos += 1;

                primary += "P";
                secondary += "P";

                continue 'a;
            }
            'Q' => {
                if characters.get(pos + 1) == Some(&'Q') {
                    pos += 1;
                }

                pos += 1;
                primary += "K";
                secondary += "K";

                continue 'a;
            }
            'R' => {
                // French such as `Rogier`, but exclude `Hochmeier`.
                if pos == characters.len()
                    && !slavo_germanic(&characters)
                    && characters.get(pos.wrapping_sub(1)) == Some(&'E')
                    && characters.get(pos - 2) == Some(&'I')
                    && characters.get(pos - 4) != Some(&'M')
                    && (characters.get(pos - 3) != Some(&'E')
                        && characters.get(pos - 3) != Some(&'A'))
                {
                    secondary += "R";
                } else {
                    primary += "R";
                    secondary += "R";
                }

                if characters.get(pos + 1) == Some(&'R') {
                    pos += 1;
                }

                pos += 1;

                continue 'a;
            }
            'S' => {
                s_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'T' => {
                t_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'V' => {
                if characters.get(pos + 1) == Some(&'V') {
                    pos += 1;
                }

                primary += "F";
                secondary += "F";
                pos += 1;

                continue 'a;
            }
            'W' => {
                w_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            'X' => {
                if !(pos == characters.len()
                    && (characters.get(pos.wrapping_sub(1)) == Some(&'U')
                        && (characters.get(pos - 2) == Some(&'A')
                            || characters.get(pos - 2) == Some(&'O'))))
                {
                    primary += "KS";
                    secondary += "KS"
                }

                if characters.get(pos + 1) == Some(&'C') || characters.get(pos + 1) == Some(&'X') {
                    pos += 1;
                }

                pos += 1;

                continue 'a;
            }
            'Z' => {
                z_case(&mut pos, &characters, &mut primary, &mut secondary);
            }
            _ => pos += 1,
        }
    }
    Ok(vec![primary, secondary])
}

fn get_char_as_string(characters: &[char], pos: usize) -> String {
    match characters.get(pos) {
        Some(c) => c.to_string(),
        None => String::new(),
    }
}

fn get_substring(characters: &[char], start: usize, end: usize) -> String {
    match characters.get(start..end) {
        Some(s) => s.iter().collect::<String>(),
        None => String::new(),
    }
}

fn germanic(characters: &[char]) -> bool {
    Word::parse(
        Rule::germanic,
        characters.iter().collect::<String>().as_str(),
    )
    .is_ok()
}

fn slavo_germanic(characters: &[char]) -> bool {
    Word::parse(
        Rule::slavo_germanic,
        characters.iter().collect::<String>().as_str(),
    )
    .is_ok()
}

fn c_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    if characters.get(pos.wrapping_sub(1)) == Some(&'A')
        && characters.get(*pos + 1) == Some(&'H')
        && characters.get(*pos + 2) != Some(&'I')
    {
        let sub_value: String = get_substring(&characters, pos.wrapping_sub(2), *pos + 4);
        if sub_value.as_str() == "BACHER" || sub_value.as_str() == "MACHER" {
            *primary += "K";
            *secondary += "K";
            *pos += 2;

            return;
        }

        if "AESAR" == get_substring(&characters, 1, 6) {
            *primary += "S";
            *secondary += "S";
            *pos += 2;

            return;
        }

        if "HIA" == get_substring(&characters, *pos + 1, *pos + 4) {
            *primary += "K";
            *secondary += "K";
            *pos += 2;

            return;
        }

        if let Some('H') = characters.get(*pos + 1) {
            if "AE" == get_substring(&characters, *pos + 2, *pos + 4) {
                *primary += "K";
                *secondary += "X";
                *pos += 2;

                return;
            }

            if Word::parse(
                Rule::initial_greek_ch,
                characters.iter().collect::<String>().as_str(),
            )
            .is_ok()
            {
                *primary += "K";
                *secondary += "K";
                *pos += 2;

                return;
            }

            let mut pos_plus_2 = String::new();

            if let Some(pp2) = characters.get(*pos + 2) {
                pos_plus_2 = pp2.to_string();
            }

            if germanic(&characters)
                || pos_plus_2 == "T"
                || pos_plus_2 == "S"
                || ((*pos == 0
                    || characters.get(pos.wrapping_sub(1)) == Some(&'A')
                    || characters.get(pos.wrapping_sub(1)) == Some(&'E')
                    || characters.get(pos.wrapping_sub(1)) == Some(&'I')
                    || characters.get(pos.wrapping_sub(1)) == Some(&'O')
                    || characters.get(pos.wrapping_sub(1)) == Some(&'U'))
                    && Word::parse(Rule::ch_for_k, pos_plus_2.as_str()).is_ok())
                || characters.get(..2) == Some(&['M', 'C'])
            {
                *primary += "K";
                *secondary += "K";
            } else if *pos == 0 {
                *primary += "X";
                *secondary += "X";
            } else {
                *primary += "X";
                *secondary += "K"
            }

            *pos += 2;
            return;
        }

        if characters.get(*pos + 1) == Some(&'Z')
            && characters.get(pos.wrapping_sub(2)..*pos) == Some(&['W', 'I'])
        {
            *primary += "S";
            *secondary += "X";
            *pos += 2;

            return;
        }

        if characters.get(*pos + 1..*pos + 4) == Some(&['C', 'I', 'A']) {
            *primary += "X";
            *secondary += "X";
            *pos += 3;

            return;
        }

        if characters.get(*pos + 1) == Some(&'C') && !(*pos == 1 && characters.get(0) == Some(&'M'))
        {
            if (characters.get(*pos + 2) == Some(&'I')
                || characters.get(*pos + 2) == Some(&'E')
                || characters.get(*pos + 2) == Some(&'H'))
                && characters.get(*pos + 2..*pos + 4) != Some(&['H', 'U'])
            {
                let val = get_substring(&characters, pos.wrapping_sub(1), *pos + 4);

                if (*pos == 1 && characters.get(pos.wrapping_sub(1)) == Some(&'M'))
                    || val == "UCCEE"
                    || val == "UCCES"
                {
                    *primary += "KS";
                    *secondary += "KS";
                } else {
                    *primary += "X";
                    *secondary += "X";
                }
            } else {
                *primary += "K";
                *secondary += "K";
                *pos += 2;
                return;
            }

            *pos += 3;
            return;
        }

        if Some(&'G') == characters.get(*pos + 1)
            || Some(&'K') == characters.get(*pos + 1)
            || Some(&'Q') == characters.get(*pos + 1)
        {
            *primary += "K";
            *secondary += "K";

            *pos += 2;
            return;
        }

        if Some(&'I') == characters.get(*pos + 1)
            && (Some(&'E') == characters.get(*pos + 2) || Some(&'O') == characters.get(*pos + 2))
        {
            *primary += "S";
            *secondary += "X";
            *pos += 2;
            return;
        }

        if Some(&'I') == characters.get(*pos + 1)
            || Some(&'E') == characters.get(*pos + 1)
            || Some(&'Y') == characters.get(*pos + 1)
        {
            *primary += "S";
            *secondary += "S";
            *pos += 2;
            return;
        }
        *primary += "K";
        *secondary += "K";

        if Some(&' ') == characters.get(*pos + 1)
            && (Some(&'C') == characters.get(*pos + 2)
                || Some(&'G') == characters.get(*pos + 2)
                || Some(&'Q') == characters.get(*pos + 2))
        {
            *pos += 3;
            return;
        }

        if characters.get(*pos + 1) == Some(&'K')
            || characters.get(*pos + 1) == Some(&'Q')
            || (characters.get(*pos + 1) == Some(&'C')
                && characters.get(*pos + 2) != Some(&'E')
                && characters.get(*pos + 2) != Some(&'I'))
        {
            *pos += 1;
        }

        *pos += 1;
    }
}

fn d_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    if characters.get(*pos + 1) == Some(&'G') {
        if characters.get(*pos + 2) == Some(&'E')
            || characters.get(*pos + 2) == Some(&'I')
            || characters.get(*pos + 2) == Some(&'Y')
        {
            *primary += "J";
            *secondary += "J";
            *pos += 3;
        } else {
            *primary += "TK";
            *secondary += "TK";
            *pos += 2;
        }

        return;
    }

    if characters.get(*pos + 1) == Some(&'T') || characters.get(*pos + 1) == Some(&'D') {
        *primary += "T";
        *secondary += "T";
        *pos += 2;

        return;
    }

    *primary += "T";
    *secondary += "T";
    *pos += 1;
}

fn g_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    if characters.get(*pos + 1) == Some(&'H') {
        let prev = get_char_as_string(&characters, pos.wrapping_sub(1));

        if *pos > 0 && Word::parse(Rule::vowels, prev.as_str()).is_err() {
            *primary += "K";
            *secondary += "K";
            *pos += 2;

            return;
        }

        // Such as `Ghislane`, `Ghiradelli`.
        if *pos == 0 {
            if characters.get(*pos + 2) == Some(&'I') {
                *primary += "J";
                *secondary += "J";
            } else {
                *primary += "K";
                *secondary += "K";
            }

            *pos += 2;

            return;
        }
        let x = characters.get(pos.wrapping_sub(2));
        let y = characters.get(pos.wrapping_sub(3));
        let z = characters.get(pos.wrapping_sub(4));

        if x == Some(&'B')
            || x == Some(&'H')
            || x == Some(&'D')
            || (y == Some(&'B') || y == Some(&'H') || y == Some(&'D'))
            || (z == Some(&'B') || z == Some(&'H'))
        {
            *pos += 2;

            return;
        }
        let s = get_char_as_string(&characters, pos.wrapping_sub(3));
        if *pos > 2
            && characters.get(pos.wrapping_sub(1)) == Some(&'U')
            && Word::parse(Rule::g_for_f, s.as_str()).is_ok()
        {
            *primary += "F";
            *secondary += "F";
        } else if *pos > 0 && characters.get(pos.wrapping_sub(1)) != Some(&'I') {
            *primary += "K";
            *secondary += "K";
        }

        *pos += 2;

        return;
    }

    if characters.get(*pos + 1) == Some(&'N') {
        if *pos == 1
            && Word::parse(Rule::vowels, get_char_as_string(&characters, 0).as_str()).is_ok()
            && !slavo_germanic(&characters)
        {
            *primary += "KN";
            *secondary += "N";
        // Not like `Cagney`.
        } else if get_substring(&characters, *pos + 2, *pos + 4) != "EY"
            && characters.get(*pos + 1) != Some(&'Y')
            && !slavo_germanic(&characters)
        {
            *primary += "N";
            *secondary += "KN"
        } else {
            *primary += "KN";
            *secondary += "KN"
        }

        *pos += 2;

        return;
    }

    // Such as `Tagliaro`.
    if get_substring(&characters, *pos + 1, *pos + 3) == "LI" && !slavo_germanic(&characters) {
        *primary += "KL";
        *secondary += "L";
        *pos += 2;

        return;
    }

    // -ges-, -gep-, -gel- at beginning.
    if *pos == 0
        && Word::parse(
            Rule::initial_g_or_for_k_or_j,
            get_substring(&characters, 1, 3).as_str(),
        )
        .is_ok()
    {
        *primary += "K";
        *secondary += "J";
        *pos += 2;

        return;
    }

    // -ger-, -gy-.
    if get_substring(&characters, *pos + 1, *pos + 3) == "ER"
        && characters.get(pos.wrapping_sub(1)) != Some(&'I')
        && characters.get(pos.wrapping_sub(1)) != Some(&'E')
        && Word::parse(
            Rule::initial_anger_exception,
            get_substring(&characters, *pos + 1, *pos + 3).as_str(),
        )
        .is_ok()
        && Word::parse(
            Rule::initial_anger_exception,
            get_substring(&characters, 0, 6).as_str(),
        )
        .is_ok()
        || (characters.get(*pos + 1) == Some(&'Y')
            && Word::parse(
                Rule::initial_g_or_for_k_or_j,
                get_char_as_string(&characters, 1).as_str(),
            )
            .is_err())
    {
        *primary += "K";
        *secondary += "J";
        *pos += 2;

        return;
    }

    // Italian such as `biaggi`.
    if characters.get(*pos + 1) == Some(&'E')
        || characters.get(*pos + 1) == Some(&'I')
        || characters.get(*pos + 1) == Some(&'Y')
        || ((characters.get(pos.wrapping_sub(1)) == Some(&'A')
            || characters.get(pos.wrapping_sub(1)) == Some(&'O'))
            && characters.get(*pos + 1) == Some(&'G')
            && characters.get(*pos + 2) == Some(&'I'))
    {
        if get_substring(&characters, *pos + 1, *pos + 3) == "ET" || germanic(&characters) {
            *primary += "K";
            *secondary += "K";
        } else {
            *primary += "J";

            // Always soft if French ending.
            if get_substring(&characters, *pos + 1, *pos + 5) == "IER " {
                *secondary += "J";
            } else {
                *secondary += "K";
            }
        }

        *pos += 2;

        return;
    }

    if characters.get(*pos + 1) == Some(&'G') {
        *pos += 1;
    }

    *pos += 1;

    *primary += "K";
    *secondary += "K";
}

fn j_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    if get_substring(&characters, *pos, *pos + 4) == "JOSE"
        || get_substring(&characters, 0, 4) == "SAN "
    {
        if get_substring(&characters, *pos, *pos + 4) == "SAN "
            || (*pos == 0 && characters.get(*pos + 4) == Some(&' '))
        {
            *primary += "H";
            *secondary += "H";
        } else {
            *primary += "J";
            *secondary += "H";
        }

        *pos += 1;

        return;
    }

    if *pos == 0 {
        *primary += "J";

        *secondary += "A";
    } else if !slavo_germanic(&characters)
        && (characters.get(*pos + 1) == Some(&'A') || characters.get(*pos + 1) == Some(&'O'))
        && Word::parse(
            Rule::vowels,
            get_char_as_string(&characters, pos.wrapping_sub(1)).as_str(),
        )
        .is_ok()
    {
        *primary += "J";
        *secondary += "H";
    } else if *pos == characters.len() {
        *primary += "J";
    } else if characters.get(pos.wrapping_sub(1)) != Some(&'S')
        && characters.get(pos.wrapping_sub(1)) != Some(&'K')
        && characters.get(pos.wrapping_sub(1)) != Some(&'L')
        && Word::parse(
            Rule::j_for_j_exception,
            get_char_as_string(&characters, *pos + 1).as_str(),
        )
        .is_err()
    {
        *primary += "J";
        *secondary += "J";
    // It could happen.
    } else if characters.get(*pos + 1) == Some(&'J') {
        *pos += 1;
    }

    *pos += 1;
}

fn s_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    // Special cases `island`, `isle`, `carlisle`, `carlysle`.
    if characters.get(*pos + 1) == Some(&'L')
        && (characters.get(pos.wrapping_sub(1)) == Some(&'I')
            || characters.get(pos.wrapping_sub(1)) == Some(&'Y'))
    {
        *pos += 1;

        return;
    }

    // Special case `sugar-`.
    if *pos == 0 && get_substring(&characters, 1, 5) == "UGAR" {
        *primary += "X";
        *secondary += "S";
        *pos += 1;

        return;
    }

    if characters.get(*pos + 1) == Some(&'H') {
        // Germanic.
        if Word::parse(
            Rule::h_for_s,
            get_substring(&characters, *pos + 1, *pos + 5).as_str(),
        )
        .is_ok()
        {
            *primary += "S";
            *secondary += "S";
        } else {
            *primary += "X";
            *secondary += "X";
        }

        *pos += 2;
        return;
    }

    if characters.get(*pos + 1) == Some(&'I')
        && (characters.get(*pos + 2) == Some(&'O') || characters.get(*pos + 2) == Some(&'A'))
    {
        if slavo_germanic(&characters) {
            *primary += "S";
            *secondary += "S";
        } else {
            *primary += "S";
            *secondary += "X";
        }

        *pos += 3;

        return;
    }

    // German & Anglicization's, such as `Smith` match `Schmidt`, `snider`
    // match `Schneider`. Also, -sz- in slavic language although in
    // hungarian it is pronounced `s`.
    if characters.get(*pos + 1) == Some(&'Z')
        || (*pos == 0
            && (characters.get(*pos + 1) == Some(&'L')
                || characters.get(*pos + 1) == Some(&'M')
                || characters.get(*pos + 1) == Some(&'N')
                || characters.get(*pos + 1) == Some(&'W')))
    {
        *primary += "S";
        *secondary += "X";

        if characters.get(*pos + 1) == Some(&'Z') {
            *pos += 1;
        }

        *pos += 1;

        return;
    }

    if characters.get(*pos + 1) == Some(&'C') {
        // Schlesinger's rule.
        if characters.get(*pos + 2) == Some(&'H') {
            let subvalue = get_substring(&characters, *pos + 3, *pos + 5);
            if Word::parse(Rule::dutch_sch, subvalue.as_str()).is_ok() {
                // Such as `schermerhorn`, `schenker`.
                if subvalue == "ER" || subvalue == "EN" {
                    *primary += "X";
                    *secondary += "SK"
                } else {
                    *primary += "SK";
                    *secondary += "SK"
                }

                *pos += 3;

                return;
            }

            if *pos == 0
                && Word::parse(Rule::vowels, get_char_as_string(&characters, 3).as_str()).is_err()
                && characters.get(3) != Some(&'W')
            {
                *primary += "X";
                *secondary += "S";
            } else {
                *primary += "X";
                *secondary += "X";
            }

            *pos += 3;

            return;
        }

        if characters.get(*pos + 2) == Some(&'I')
            || characters.get(*pos + 2) == Some(&'E')
            || characters.get(*pos + 2) == Some(&'Y')
        {
            *primary += "S";
            *secondary += "S";
            *pos += 3;
            return;
        }

        *primary += "SK";
        *secondary += "SK";
        *pos += 3;

        return;
    }

    let subvalue = get_substring(&characters, pos.wrapping_sub(2), *pos);

    // French such as `resnais`, `artois`.
    if *pos == characters.len() && (subvalue == "AI" || subvalue == "OI") {
        *secondary += "S";
    } else {
        *primary += "S";
        *secondary += "S";
    }

    if characters.get(*pos + 1) == Some(&'S') {
        *pos += 1;
    }

    *pos += 1;
}

fn t_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    if characters.get(*pos + 1) == Some(&'I')
        && characters.get(*pos + 2) == Some(&'O')
        && characters.get(*pos + 3) == Some(&'N')
    {
        *primary += "X";
        *secondary += "X";
        *pos += 3;

        return;
    }

    if (characters.get(*pos + 1) == Some(&'I') && characters.get(*pos + 2) == Some(&'A'))
        || (characters.get(*pos + 1) == Some(&'C') && characters.get(*pos + 2) == Some(&'H'))
    {
        *primary += "X";
        *secondary += "X";
        *pos += 3;

        return;
    }

    if characters.get(*pos + 1) == Some(&'H')
        || (characters.get(*pos + 1) == Some(&'T') && characters.get(*pos + 2) == Some(&'H'))
    {
        // Special case `Thomas`, `Thames` or Germanic.
        if germanic(&characters)
            || ((characters.get(*pos + 2) == Some(&'O') || characters.get(*pos + 2) == Some(&'A'))
                && characters.get(*pos + 3) == Some(&'M'))
        {
            *primary += "T";
            *secondary += "T";
        } else {
            *primary += "0";
            *secondary += "T";
        }

        *pos += 2;

        return;
    }

    if characters.get(*pos + 1) == Some(&'T') || characters.get(*pos + 1) == Some(&'D') {
        *pos += 1;
    }

    *pos += 1;
    *primary += "T";
    *secondary += "T";
}
fn w_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    // Can also be in middle of word (as already taken care of for initial).
    if characters.get(*pos + 1) == Some(&'R') {
        *primary += "R";
        *secondary += "R";
        *pos += 2;

        return;
    }

    if *pos == 0 {
        // `Wasserman` should match `Vasserman`.
        if Word::parse(
            Rule::vowels,
            get_char_as_string(&characters, *pos + 1).as_str(),
        )
        .is_ok()
        {
            *primary += "A";
            *secondary += "F";
        } else if characters.get(*pos + 1) == Some(&'H') {
            // Need `Uomo` to match `Womo`.
            *primary += "A";
            *secondary += "A";
        }
    }

    // `Arnow` should match `Arnoff`.
    if ((characters.get(pos.wrapping_sub(1))  ==Some(&'E') || characters.get(pos.wrapping_sub(1))  == Some(&'O')) &&
            characters.get(*pos + 1) == Some(&'S') &&
            characters.get(*pos + 2) == Some(&'K') &&
            (characters.get(*pos + 3) == Some(&'I') || characters.get(*pos + 3) == Some(&'Y'))) ||
          // Maybe a bug? Shouldn't this be general Germanic?
          get_substring(&characters, 0, 3) == "SCH" ||
          (*pos == characters.len() && Word::parse(Rule::vowels, get_char_as_string(&characters, pos.wrapping_sub(1)).as_str()).is_ok())
    {
        *secondary += "F";
        *pos += 1;

        return;
    }

    // Polish such as `Filipowicz`.
    if characters.get(*pos + 1) == Some(&'I')
        && (characters.get(*pos + 2) == Some(&'C') || characters.get(*pos + 2) == Some(&'T'))
        && characters.get(*pos + 3) == Some(&'Z')
    {
        *primary += "TS";
        *secondary += "FX";
        *pos += 4;

        return;
    }

    *pos += 1;
}
fn z_case(pos: &mut usize, characters: &[char], primary: &mut String, secondary: &mut String) {
    // Chinese pinyin such as `Zhao`.
    if characters.get(*pos + 1) == Some(&'H') {
        *primary += "J";
        *secondary += "J";
        *pos += 2;

        return;
    } else if (characters.get(*pos + 1) == Some(&'Z')
        && (characters.get(*pos + 2) == Some(&'A')
            || characters.get(*pos + 2) == Some(&'I')
            || characters.get(*pos + 2) == Some(&'O')))
        || (slavo_germanic(&characters)
            && *pos > 0
            && characters.get(pos.wrapping_sub(1)) != Some(&'T'))
    {
        *primary += "S";
        *secondary += "TS"
    } else {
        *primary += "S";
        *secondary += "S";
    }

    if characters.get(*pos + 1) == Some(&'Z') {
        *pos += 1;
    }

    *pos += 1;
}
