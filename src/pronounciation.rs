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

    let slavo_germanic: bool = Word::parse(Rule::slavo_germanic, word.as_str()).is_ok();
    let germanic = Word::parse(Rule::germanic, word.as_str()).is_ok();
    let characters = word.chars().collect::<Vec<char>>();
    let mut pos: usize = 0;

    if Word::parse(Rule::initial_exceptions, word.as_str()).is_ok() {
        pos += 1;
    } else if let Some('X') = characters.first() {
        pos += 1
    }

    'a: while let Some(c) = characters.get(pos) {
        let previous: usize = pos.wrapping_sub(1);

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
                if characters.get(previous) == Some(&'A')
                    && characters.get(pos + 1) == Some(&'H')
                    && characters.get(pos + 2) != Some(&'I')
                {
                    if let Some(s) = characters.get(pos.wrapping_sub(2)..pos + 4) {
                        let sub_value: String = s.iter().collect();
                        if sub_value.as_str() == "BACHER" || sub_value.as_str() == "MACHER" {
                            primary += "K";
                            secondary += "K";
                            pos += 2;

                            continue 'a;
                        }
                    }
                }

                if let Some(s) = characters.get(1..6) {
                    if "AESAR" == s.iter().collect::<String>() {
                        primary += "S";
                        secondary += "S";
                        pos += 2;

                        continue 'a;
                    }
                }

                if "HIA"
                    == characters
                        .get(pos + 1..pos + 4)
                        .ok_or(String::new())
                        .unwrap()
                        .iter()
                        .collect::<String>()
                {
                    primary += "K";
                    secondary += "K";
                    pos += 2;

                    continue 'a;
                }

                if let Some('H') = characters.get(pos + 1) {
                    if let Some(s) = characters.get(pos + 2..pos + 4) {
                        if "AE" == s.iter().collect::<String>() {
                            primary += "K";
                            secondary += "X";
                            pos += 2;

                            continue 'a;
                        }

                        if Word::parse(Rule::initial_greek_ch, word.as_str()).is_ok() {
                            primary += "K";
                            secondary += "K";
                            pos += 2;

                            continue 'a;
                        }

                        let mut pos_plus_2 = String::new();

                        if let Some(pp2) = characters.get(pos + 2) {
                            pos_plus_2 = pp2.to_string();
                        }

                        if germanic
                            || pos_plus_2 == "T"
                            || pos_plus_2 == "S"
                            || ((pos == 0
                                || characters.get(pos.wrapping_sub(1)) == Some(&'A')
                                || characters.get(pos.wrapping_sub(1)) == Some(&'E')
                                || characters.get(pos.wrapping_sub(1)) == Some(&'I')
                                || characters.get(pos.wrapping_sub(1)) == Some(&'O')
                                || characters.get(pos.wrapping_sub(1)) == Some(&'U'))
                                && Word::parse(Rule::ch_for_k, pos_plus_2.as_str()).is_ok())
                            || characters.get(..2) == Some(&['M', 'C'])
                        {
                            primary += "K";
                            secondary += "K";
                        } else if pos == 0 {
                            primary += "X";
                            secondary += "X";
                        } else {
                            primary += "X";
                            secondary += "K"
                        }
                    }

                    pos += 2;
                    continue 'a;
                }

                if characters.get(pos + 1) == Some(&'Z')
                    && characters.get(pos.wrapping_sub(2)..pos) == Some(&['W', 'I'])
                {
                    primary += "S";
                    secondary += "X";
                    pos += 2;

                    continue 'a;
                }

                if characters.get(pos + 1..pos + 4) == Some(&['C', 'I', 'A']) {
                    primary += "X";
                    secondary += "X";
                    pos += 3;

                    continue 'a;
                }

                if characters.get(pos + 1) == Some(&'C')
                    && !(pos == 1 && characters.get(0) == Some(&'M'))
                {
                    if (characters.get(pos + 2) == Some(&'I')
                        || characters.get(pos + 2) == Some(&'E')
                        || characters.get(pos + 2) == Some(&'H'))
                        && characters.get(pos + 2..pos + 4) != Some(&['H', 'U'])
                    {
                        let val = characters
                            .get(pos.wrapping_sub(1)..pos + 4)
                            .ok_or(String::new())
                            .unwrap()
                            .iter()
                            .collect::<String>();

                        if (pos == 1 && characters.get(pos.wrapping_sub(1)) == Some(&'M'))
                            || val == "UCCEE"
                            || val == "UCCES"
                        {
                            primary += "KS";
                            secondary += "KS";
                        } else {
                            primary += "X";
                            secondary += "X";
                        }
                    } else {
                        primary += "K";
                        secondary += "K";
                        pos += 2;
                        continue 'a;
                    }

                    pos += 3;
                    continue 'a;
                }

                if Some(&'G') == characters.get(pos + 1)
                    || Some(&'K') == characters.get(pos + 1)
                    || Some(&'Q') == characters.get(pos + 1)
                {
                    primary += "K";
                    secondary += "K";

                    pos += 2;
                    continue 'a;
                }

                if Some(&'I') == characters.get(pos + 1)
                    && (Some(&'E') == characters.get(pos + 2)
                        || Some(&'O') == characters.get(pos + 2))
                {
                    primary += "S";
                    secondary += "X";
                    pos += 2;
                    continue 'a;
                }

                if Some(&'I') == characters.get(pos + 1)
                    || Some(&'E') == characters.get(pos + 1)
                    || Some(&'Y') == characters.get(pos + 1)
                {
                    primary += "S";
                    secondary += "S";
                    pos += 2;
                    continue 'a;
                }

                primary += "K";
                secondary += "K";

                if Some(&' ') == characters.get(pos + 1)
                    && (Some(&'C') == characters.get(pos + 2)
                        || Some(&'G') == characters.get(pos + 2)
                        || Some(&'Q') == characters.get(pos + 2))
                {
                    pos += 3;
                    continue 'a;
                }

                if characters.get(pos + 1) == Some(&'K')
                    || characters.get(pos + 1) == Some(&'Q')
                    || (characters.get(pos + 1) == Some(&'C')
                        && characters.get(pos + 2) != Some(&'E')
                        && characters.get(pos + 2) != Some(&'I'))
                {
                    pos += 1;
                }

                pos += 1;

                continue 'a;
            }
      'D' => {
        if (characters.get(pos + 1) == Some(&'G')) {
          if (nextcharacters.get(pos + 1) == Some(&'E') || nextcharacters.get(pos + 1) == Some(&'I') || nextcharacters.get(pos + 1) == Some(&'Y')) {
            primary += "J";
            secondary += "J";
            pos += 3;
          } else {
            primary += "TK";
            secondary += "TK";
            pos += 2;
          }

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'T') || characters.get(pos + 1) == Some(&'D')) {
          primary += "T";
          secondary += "T";
          pos += 2;

          continue 'a;
        }

        primary += "T";
        secondary += "T";
        pos += 1;

        continue 'a;
      }
      'F' => {
        if (characters.get(pos + 1) == Some(&'F')) {
          pos += 1;
        }

        pos += 1;
        primary += "F";
        secondary += "F";

        continue 'a;}
      'G' => {
        if (characters.get(pos + 1) == Some(&'H')) {
          if (pos > 0 && !vowels.test(prev)) {
            primary += "K";
            secondary += "K";
            pos += 2;

            continue 'a;
          }

          // Such as `Ghislane`, `Ghiradelli`.
          if (pos == 0) {
            if (nextcharacters.get(pos + 1) == Some(&'I')) {
              primary += "J";
              secondary += "J";
            } else {
              primary += "K";
              secondary += "K";
            }

            pos += 2;

            continue 'a;
          }

          // Parker's rule (with some further refinements).
          if (
            // Such as `Hugh`.  The comma is not a bug.
            ((subvalue = characters[pos - 2]),
            subvalue == Some(&'B') || subvalue == Some(&'H') || subvalue == Some(&'D')) ||
            // Such as `bough`.  The comma is not a bug.
            ((subvalue = characters[pos - 3]),
            subvalue == Some(&'B') || subvalue == Some(&'H') || subvalue == Some(&'D')) ||
            // Such as `Broughton`.  The comma is not a bug.
            ((subvalue = characters[pos - 4]),
            subvalue == Some(&'B') || subvalue == Some(&'H'))
          ) {
            pos += 2;

            continue 'a;
          }

          // Such as `laugh`, `McLaughlin`, `cough`, `gough`, `rough`, `tough`.
          if (pos > 2 && prev == Some(&'U') && gForF.test(characters[pos - 3])) {
            primary += "F";
            secondary += "F";
          } else if (pos > 0 && prev != Some(&'I')) {
            primary += "K";
            secondary += "K";
          }

          pos += 2;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'N')) {
          if (pos == 1 && vowels.test(characters[0]) && !isSlavoGermanic) {
            primary += "KN";
            secondary += "N";
            // Not like `Cagney`.
          } else if (
            characters.get(pos+2..pos+4).ok_or(String::new()).unwrap().collect::<String>()!= "EY" &&
            value.slice(pos + 1) != Some(&'Y') &&
            !isSlavoGermanic
          ) {
            primary += "N";
            secondary += "KN"
          } else {
            primary += "KN";
            secondary += "KN"
          }

          pos += 2;

          continue 'a;
        }

        // Such as `Tagliaro`.
        if (characters.get(pos+1..pos+3).ok_or(String::new()).unwrap().collect::<String>()== "LI" && !isSlavoGermanic) {
          primary += "KL";
          secondary += "L";
          pos += 2;

          continue 'a;
        }

        // -ges-, -gep-, -gel- at beginning.
        if (pos == 0 && initialGForKj.test(value.slice(1, 3))) {
          primary += "K";
          secondary += "J";
          pos += 2;

          continue 'a;
        }

        // -ger-, -gy-.
        if (
          (characters.get(pos+1..pos+3).ok_or(String::new()).unwrap().collect::<String>()== "ER" &&
            prev != Some(&'I') &&
            prev != Some(&'E') &&
            !initialAngerException.test(value.slice(0, 6))) ||
          (characters.get(pos + 1) == Some(&'Y') && !gForKj.test(prev))
        ) {
          primary += "K";
          secondary += "J";
          pos += 2;

          continue 'a;
        }

        // Italian such as `biaggi`.
        if (
          characters.get(pos + 1) == Some(&'E') ||
          characters.get(pos + 1) == Some(&'I') ||
          characters.get(pos + 1) == Some(&'Y') ||
          ((prev == Some(&'A') || prev == Some(&'O')) && characters.get(pos + 1) == Some(&'G') && nextcharacters.get(pos + 1) == Some(&'I'))
        ) {
          // Obvious Germanic.
          if (characters.get(pos+1..pos+3).ok_or(String::new()).unwrap().collect::<String>()== "ET" || isGermanic) {
            primary += "K";
            secondary += "K";
          } else {
            primary += "J";

            // Always soft if French ending.
            if (characters.get(pos+1..pos+5).ok_or(String::new()).unwrap().collect::<String>()== "IER ") {
              secondary += "J";
            } else {
              secondary += "K";
            }
          }

          pos += 2;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'G')) {
          pos += 1;
        }

        pos += 1;

        primary += "K";
        secondary += "K";

        continue 'a;}
      'H' => {
        // Only keep if first & before vowel or btw. 2 vowels.
        if (vowels.test(next) && (pos == 0 || vowels.test(prev))) {
          primary += "H";
          secondary += "H";

          pos += 1;
        }

        pos += 1;

        continue 'a;}
      'J' =>{
        // Obvious Spanish, `jose`, `San Jacinto`.
        if (
          value.slice(pos, pos + 4) == "JOSE" ||
          value.slice(0, 4) == "SAN "
        ) {
          if (
            value.slice(0, 4) == "SAN " ||
            (pos == 0 && characters[pos + 4] == ' ')
          ) {
            primary += "H";
            secondary += "H";
          } else {
            primary += "J";
            secondary += "H";
          }

          pos += 1;

          continue 'a;
        }

        if (
          pos == 0
          // Bug: unreachable (see previous statement).
          // && value.slice(pos, pos + 4) != "JOSE".
        ) {
          primary += "J";

          // Such as `Yankelovich` or `Jankelowicz`.
          secondary += "A";
          // Spanish pron. of such as `bajador`.
        } else if (
          !isSlavoGermanic &&
          (characters.get(pos + 1) == Some(&'A') || characters.get(pos + 1) == Some(&'O')) &&
          vowels.test(prev)
        ) {
          primary += "J";
          secondary += "H";
        } else if (pos == last) {
          primary += "J";
        } else if (
          prev != Some(&'S') &&
          prev != Some(&'K') &&
          prev != Some(&'L') &&
          !jForJException.test(next)
        ) {
          primary += "J";
          secondary += "J";
          // It could happen.
        } else if (characters.get(pos + 1) == Some(&'J')) {
          pos += 1;
        }

        pos += 1;

        continue 'a;}
      'K' =>{
        if (characters.get(pos + 1) == Some(&'K')) {
          pos += 1;
        }

        primary += "K";
        secondary += "K";
        pos += 1;

        continue 'a;}
      'L' => {
        if (characters.get(pos + 1) == Some(&'L')) {
          // Spanish such as `cabrillo`, `gallegos`.
          if (
            (pos == length - 3 &&
              ((prev == Some(&'A') && nextcharacters.get(pos + 1) == Some(&'E')) ||
                (prev == Some(&'I') && (nextcharacters.get(pos + 1) == Some(&'O') || nextcharacters.get(pos + 1) == Some(&'A'))))) ||
            (prev == Some(&'A') &&
              nextcharacters.get(pos + 1) == Some(&'E') &&
              (characters[last] == Some(&'A') ||
                characters[last] == Some(&'O') ||
                alle.test(value.slice(last - 1, length))))
          ) {
            primary += "L";
            pos += 2;

            continue 'a;
          }

          pos += 1;
        }

        primary += "L";
        secondary += "L";
        pos += 1;

        continue 'a;}
      'M' => {
        if (
          characters.get(pos + 1) == Some(&'M') ||
          // Such as `dumb`, `thumb`.
          (prev == Some(&'U') &&
            characters.get(pos + 1) == Some(&'B') &&
            (pos + 1 == last || characters.get(pos+2..pos+4).ok_or(String::new()).unwrap().collect::<String>()== "ER"))
        ) {
          pos += 1;
        }

        pos += 1;
        primary += "M";
        secondary += "M";

        continue 'a;}
      'N' => {
        if (characters.get(pos + 1) == Some(&'N')) {
          pos += 1;
        }

        pos += 1;
        primary += "N";
        secondary += "N";

        continue 'a; }
      'P' => {
        if (characters.get(pos + 1) == Some(&'H')) {
          primary += "F";
          secondary += "F";
          pos += 2;

          continue 'a;
        }

        // Also account for `campbell` and `raspberry`.
        subvalue = next

        if (subvalue == Some(&'P') || subvalue == Some(&'B')) {
          pos += 1;
        }

        pos += 1;

        primary += "P";
        secondary += "P";

        continue 'a;}
      'Q' => {
        if (characters.get(pos + 1) == Some(&'Q')) {
          pos += 1;
        }

        pos += 1;
        primary += "K";
        secondary += "K";

        continue 'a; }
      'R' => {
        // French such as `Rogier`, but exclude `Hochmeier`.
        if (
          pos == last &&
          !isSlavoGermanic &&
          prev == Some(&'E') &&
          characters[pos - 2] == Some(&'I') &&
          characters[pos - 4] != Some(&'M') &&
          (characters[pos - 3] != Some(&'E') && characters[pos - 3] != Some(&'A'))
        ) {
          secondary += "R";
        } else {
          primary += "R";
          secondary += "R";
        }

        if (characters.get(pos + 1) == Some(&'R')) {
          pos += 1;
        }

        pos += 1;

        continue 'a;}
      'S' => {
        // Special cases `island`, `isle`, `carlisle`, `carlysle`.
        if (characters.get(pos + 1) == Some(&'L') && (prev == Some(&'I') || prev == Some(&'Y'))) {
          pos += 1;

          continue 'a;
        }

        // Special case `sugar-`.
        if (pos == 0 && value.slice(1, 5) == "UGAR") {
          primary += "X";
          secondary += "S";
          pos += 1;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'H')) {
          // Germanic.
          if (hForS.test(value.slice(pos + 1, pos + 5))) {
            primary += "S";
            secondary += "S";
          } else {
            primary += "X";
            secondary += "X";
          }

          pos += 2;
          continue 'a;
        }

        if (
          characters.get(pos + 1) == Some(&'I') &&
          (nextcharacters.get(pos + 1) == Some(&'O') || nextcharacters.get(pos + 1) == Some(&'A'))
          // Bug: Already covered by previous branch
          // || value.slice(pos, pos + 4) == "SIAN"
        ) {
          if (isSlavoGermanic) {
            primary += "S";
            secondary += "S";
          } else {
            primary += "S";
            secondary += "X";
          }

          pos += 3;

          continue 'a;
        }

        // German & Anglicization's, such as `Smith` match `Schmidt`, `snider`
        // match `Schneider`. Also, -sz- in slavic language although in
        // hungarian it is pronounced `s`.
        if (
          characters.get(pos + 1) == Some(&'Z') ||
          (pos == 0 &&
            (characters.get(pos + 1) == Some(&'L') || characters.get(pos + 1) == Some(&'M') || characters.get(pos + 1) == Some(&'N') || characters.get(pos + 1) == Some(&'W')))
        ) {
          primary += "S";
          secondary += "X";

          if (characters.get(pos + 1) == Some(&'Z')) {
            pos += 1;
          }

          pos += 1;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'C')) {
          // Schlesinger's rule.
          if (nextcharacters.get(pos + 1) == Some(&'H')) {
            subvalue = value.slice(pos + 3, pos + 5)

            // Dutch origin, such as `school`, `schooner`.
            if (dutchSch.test(subvalue)) {
              // Such as `schermerhorn`, `schenker`.
              if (subvalue == "ER" || subvalue == "EN") {
                primary += "X";
                secondary += "SK"
              } else {
                primary += "SK";
                secondary += "SK"
              }

              pos += 3;

              continue 'a;
            }

            if (
              pos == 0 &&
              !vowels.test(characters[3]) &&
              characters[3] != Some(&'W')
            ) {
              primary += "X";
              secondary += "S";
            } else {
              primary += "X";
              secondary += "X";
            }

            pos += 3;

            continue 'a;
          }

          if (nextcharacters.get(pos + 1) == Some(&'I') || nextcharacters.get(pos + 1) == Some(&'E') || nextcharacters.get(pos + 1) == Some(&'Y')) {
            primary += "S";
            secondary += "S";
            pos += 3;
            continue 'a;
          }

          primary += "SK";
          secondary += "SK"
          pos += 3;

          continue 'a;
        }

        subvalue = value.slice(pos - 2, pos)

        // French such as `resnais`, `artois`.
        if (pos == last && (subvalue == "AI" || subvalue == "OI")) {
          secondary += "S";
        } else {
          primary += "S";
          secondary += "S";
        }

        if (
          characters.get(pos + 1) == Some(&'S')
          // Bug: already taken care of by `German & Anglicization's` above:
          // || characters.get(pos + 1) == Some(&'Z')
        ) {
          pos += 1;
        }

        pos += 1;

        continue 'a; }
      'T' => {
        if (characters.get(pos + 1) == Some(&'I') && nextcharacters.get(pos + 1) == Some(&'O') && characters[pos + 3] == Some(&'N')) {
          primary += "X";
          secondary += "X";
          pos += 3;

          continue 'a;
        }

        subvalue = value.slice(pos + 1, pos + 3)

        if (
          (characters.get(pos + 1) == Some(&'I') && nextcharacters.get(pos + 1) == Some(&'A')) ||
          (characters.get(pos + 1) == Some(&'C') && nextcharacters.get(pos + 1) == Some(&'H'))
        ) {
          primary += "X";
          secondary += "X";
          pos += 3;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'H') || (characters.get(pos + 1) == Some(&'T') && nextcharacters.get(pos + 1) == Some(&'H'))) {
          // Special case `Thomas`, `Thames` or Germanic.
          if (
            isGermanic ||
            ((nextcharacters.get(pos + 1) == Some(&'O') || nextcharacters.get(pos + 1) == Some(&'A')) &&
              characters[pos + 3] == Some(&'M'))
          ) {
            primary += "T";
            secondary += "T";
          } else {
            primary += "0";
            secondary += "T";
          }

          pos += 2;

          continue 'a;
        }

        if (characters.get(pos + 1) == Some(&'T') || characters.get(pos + 1) == Some(&'D')) {
          pos += 1;
        }

        pos += 1;
        primary += "T";
        secondary += "T";

        continue 'a;}
      'V' => {
        if (characters.get(pos + 1) == Some(&'V')) {
          pos += 1;
        }

        primary += "F";
        secondary += "F";
        pos += 1;

        continue 'a;}
      'W' => {
        // Can also be in middle of word (as already taken care of for initial).
        if (characters.get(pos + 1) == Some(&'R')) {
          primary += "R";
          secondary += "R";
          pos += 2;

          continue 'a;
        }

        if (pos == 0) {
          // `Wasserman` should match `Vasserman`.
          if (vowels.test(next)) {
            primary += "A";
            secondary += "F";
          } else if (characters.get(pos + 1) == Some(&'H')) {
            // Need `Uomo` to match `Womo`.
            primary += "A";
            secondary += "A";
          }
        }

        // `Arnow` should match `Arnoff`.
        if (
          ((prev == Some(&'E') || prev == Some(&'O')) &&
            characters.get(pos + 1) == Some(&'S') &&
            nextcharacters.get(pos + 1) == Some(&'K') &&
            (characters[pos + 3] == Some(&'I') || characters[pos + 3] == Some(&'Y'))) ||
          // Maybe a bug? Shouldn't this be general Germanic?
          value.slice(0, 3) == "SCH" ||
          (pos == last && vowels.test(prev))
        ) {
          secondary += "F";
          pos += 1;

          continue 'a;
        }

        // Polish such as `Filipowicz`.
        if (
          characters.get(pos + 1) == Some(&'I') &&
          (nextcharacters.get(pos + 1) == Some(&'C') || nextcharacters.get(pos + 1) == Some(&'T')) &&
          characters[pos + 3] == Some(&'Z')
        ) {
          primary += "TS";
          secondary += "FX"
          pos += 4;

          continue 'a;
        }

        pos += 1;

        continue 'a;}
      'X' => {
        // French such as `breaux`.
        if (
          !(
            pos == last &&
            // Bug: IAU and EAU also match by AU
            // (/IAU|EAU/.test(value.slice(pos - 3, pos))) ||
            (prev == Some(&'U') &&
              (characters[pos - 2] == Some(&'A') || characters[pos - 2] == Some(&'O')))
          )
        ) {
          primary += "KS";
          secondary += "KS"
        }

        if (characters.get(pos + 1) == Some(&'C') || characters.get(pos + 1) == Some(&'X')) {
          pos += 1;
        }

        pos += 1;

        continue 'a;}
      'Z' => {
        // Chinese pinyin such as `Zhao`.
        if (characters.get(pos + 1) == Some(&'H')) {
          primary += "J";
          secondary += "J";
          pos += 2;

          continue 'a;
        } else if (
          (characters.get(pos + 1) == Some(&'Z') &&
            (nextcharacters.get(pos + 1) == Some(&'A') || nextcharacters.get(pos + 1) == Some(&'I') || nextcharacters.get(pos + 1) == Some(&'O'))) ||
          (isSlavoGermanic && pos > 0 && prev != Some(&'T'))
        ) {
          primary += "S";
          secondary += "TS"
        } else {
          primary += "S";
          secondary += "S";
        }

        if (characters.get(pos + 1) == Some(&'Z')) {
          pos += 1;
        }

        pos += 1;

        continue 'a;}
            _ => { pos += 1}
        }
    }
    Ok(vec![])
}

//   }

//   return [primary, secondary]
// }
