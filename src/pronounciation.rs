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

        let next: usize = pos + 1;

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

                if let Some('B') = characters.get(next) {
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
                    && characters.get(next) == Some(&'H')
                    && characters.get(next + 1) != Some(&'I')
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

                if let Some(s) = characters.get(pos + 1..pos + 4) {
                    if "HIA" == s.iter().collect::<String>() {
                        primary += "K";
                        secondary += "K";
                        pos += 2;

                        continue 'a;
                    }

                    if "HAE" == s.iter().collect::<String>() {
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

                    if let Some(s) = characters.get(pos.wrapping_sub(2)..pos + 4) {
                        if Word::parse(Rule::greek_ch, s.iter().collect::<String>().as_str())
                            .is_ok()
                        {}
                    }
                }
            }

            _ => {}
        }
    }
    Ok(vec![])
}
//           // Germanic, Greek, or otherwise `CH` for `KH` sound.
//           if (
//             isGermanic ||
//             // Such as 'architect' but not 'arch', orchestra', 'orchid'.
//             greekCh.test(value.slice(index - 2, index + 4)) ||
//             (nextnext === 'T' || nextnext === 'S') ||
//             ((index === 0 ||
//               prev === 'A' ||
//               prev === 'E' ||
//               prev === 'O' ||
//               prev === 'U') &&
//               // Such as `wachtler`, `weschsler`, but not `tichner`.
//               chForKh.test(nextnext))
//           ) {
//             primary += 'K'
//             secondary += 'K'
//           } else if (index === 0) {
//             primary += 'X'
//             secondary += 'X'
//             // Such as 'McHugh'.
//           } else if (value.slice(0, 2) === 'MC') {
//             // Bug? Why matching absolute? what about McHiccup?
//             primary += 'K'
//             secondary += 'K'
//           } else {
//             primary += 'X'
//             secondary += 'K'
//           }

//           index += 2

//           break
//         }

//         // Such as `Czerny`.
//         if (next === 'Z' && value.slice(index - 2, index) !== 'WI') {
//           primary += 'S'
//           secondary += 'X'
//           index += 2

//           break
//         }

//         // Such as `Focaccia`.
//         if (value.slice(index + 1, index + 4) === 'CIA') {
//           primary += 'X'
//           secondary += 'X'
//           index += 3

//           break
//         }

//         // Double `C`, but not `McClellan`.
//         if (next === 'C' && !(index === 1 && characters[0] === 'M')) {
//           // Such as `Bellocchio`, but not `Bacchus`.
//           if (
//             (nextnext === 'I' || nextnext === 'E' || nextnext === 'H') &&
//             value.slice(index + 2, index + 4) !== 'HU'
//           ) {
//             subvalue = value.slice(index - 1, index + 4)

//             // Such as `Accident`, `Accede`, `Succeed`.
//             if (
//               (index === 1 && prev === 'A') ||
//               subvalue === 'UCCEE' ||
//               subvalue === 'UCCES'
//             ) {
//               primary += 'KS'
//               secondary += 'KS'
//               // Such as `Bacci`, `Bertucci`, other Italian.
//             } else {
//               primary += 'X'
//               secondary += 'X'
//             }

//             index += 3

//             break
//           } else {
//             // Pierce's rule.
//             primary += 'K'
//             secondary += 'K'
//             index += 2

//             break
//           }
//         }

//         if (next === 'G' || next === 'K' || next === 'Q') {
//           primary += 'K'
//           secondary += 'K'
//           index += 2

//           break
//         }

//         // Italian.
//         if (
//           next === 'I' &&
//           // Bug: The original algorithm also calls for A (as in CIA), which is
//           // already taken care of above.
//           (nextnext === 'E' || nextnext === 'O')
//         ) {
//           primary += 'S'
//           secondary += 'X'
//           index += 2

//           break
//         }

//         if (next === 'I' || next === 'E' || next === 'Y') {
//           primary += 'S'
//           secondary += 'S'
//           index += 2

//           break
//         }

//         primary += 'K'
//         secondary += 'K'

//         // Skip two extra characters ahead in `Mac Caffrey`, `Mac Gregor`.
//         if (
//           next === ' ' &&
//           (nextnext === 'C' || nextnext === 'G' || nextnext === 'Q')
//         ) {
//           index += 3
//           break
//         }

//         // Bug: Already covered above.
//         // if (
//         //   next === 'K' ||
//         //   next === 'Q' ||
//         //   (next === 'C' && nextnext !== 'E' && nextnext !== 'I')
//         // ) {
//         //   index++;
//         // }

//         index++

//         break
//       'D' =>
//         if (next === 'G') {
//           // Such as `edge`.
//           if (nextnext === 'E' || nextnext === 'I' || nextnext === 'Y') {
//             primary += 'J'
//             secondary += 'J'
//             index += 3
//             // Such as `Edgar`.
//           } else {
//             primary += 'TK'
//             secondary += 'TK'
//             index += 2
//           }

//           break
//         }

//         if (next === 'T' || next === 'D') {
//           primary += 'T'
//           secondary += 'T'
//           index += 2

//           break
//         }

//         primary += 'T'
//         secondary += 'T'
//         index++

//         break
//       'F' =>
//         if (next === 'F') {
//           index++
//         }

//         index++
//         primary += 'F'
//         secondary += 'F'

//         break
//       'G' =>
//         if (next === 'H') {
//           if (index > 0 && !vowels.test(prev)) {
//             primary += 'K'
//             secondary += 'K'
//             index += 2

//             break
//           }

//           // Such as `Ghislane`, `Ghiradelli`.
//           if (index === 0) {
//             if (nextnext === 'I') {
//               primary += 'J'
//               secondary += 'J'
//             } else {
//               primary += 'K'
//               secondary += 'K'
//             }

//             index += 2

//             break
//           }

//           // Parker's rule (with some further refinements).
//           if (
//             // Such as `Hugh`.  The comma is not a bug.
//             ((subvalue = characters[index - 2]),
//             subvalue === 'B' || subvalue === 'H' || subvalue === 'D') ||
//             // Such as `bough`.  The comma is not a bug.
//             ((subvalue = characters[index - 3]),
//             subvalue === 'B' || subvalue === 'H' || subvalue === 'D') ||
//             // Such as `Broughton`.  The comma is not a bug.
//             ((subvalue = characters[index - 4]),
//             subvalue === 'B' || subvalue === 'H')
//           ) {
//             index += 2

//             break
//           }

//           // Such as `laugh`, `McLaughlin`, `cough`, `gough`, `rough`, `tough`.
//           if (index > 2 && prev === 'U' && gForF.test(characters[index - 3])) {
//             primary += 'F'
//             secondary += 'F'
//           } else if (index > 0 && prev !== 'I') {
//             primary += 'K'
//             secondary += 'K'
//           }

//           index += 2

//           break
//         }

//         if (next === 'N') {
//           if (index === 1 && vowels.test(characters[0]) && !isSlavoGermanic) {
//             primary += 'KN'
//             secondary += 'N'
//             // Not like `Cagney`.
//           } else if (
//             value.slice(index + 2, index + 4) !== 'EY' &&
//             value.slice(index + 1) !== 'Y' &&
//             !isSlavoGermanic
//           ) {
//             primary += 'N'
//             secondary += 'KN'
//           } else {
//             primary += 'KN'
//             secondary += 'KN'
//           }

//           index += 2

//           break
//         }

//         // Such as `Tagliaro`.
//         if (value.slice(index + 1, index + 3) === 'LI' && !isSlavoGermanic) {
//           primary += 'KL'
//           secondary += 'L'
//           index += 2

//           break
//         }

//         // -ges-, -gep-, -gel- at beginning.
//         if (index === 0 && initialGForKj.test(value.slice(1, 3))) {
//           primary += 'K'
//           secondary += 'J'
//           index += 2

//           break
//         }

//         // -ger-, -gy-.
//         if (
//           (value.slice(index + 1, index + 3) === 'ER' &&
//             prev !== 'I' &&
//             prev !== 'E' &&
//             !initialAngerException.test(value.slice(0, 6))) ||
//           (next === 'Y' && !gForKj.test(prev))
//         ) {
//           primary += 'K'
//           secondary += 'J'
//           index += 2

//           break
//         }

//         // Italian such as `biaggi`.
//         if (
//           next === 'E' ||
//           next === 'I' ||
//           next === 'Y' ||
//           ((prev === 'A' || prev === 'O') && next === 'G' && nextnext === 'I')
//         ) {
//           // Obvious Germanic.
//           if (value.slice(index + 1, index + 3) === 'ET' || isGermanic) {
//             primary += 'K'
//             secondary += 'K'
//           } else {
//             primary += 'J'

//             // Always soft if French ending.
//             if (value.slice(index + 1, index + 5) === 'IER ') {
//               secondary += 'J'
//             } else {
//               secondary += 'K'
//             }
//           }

//           index += 2

//           break
//         }

//         if (next === 'G') {
//           index++
//         }

//         index++

//         primary += 'K'
//         secondary += 'K'

//         break
//       'H' =>
//         // Only keep if first & before vowel or btw. 2 vowels.
//         if (vowels.test(next) && (index === 0 || vowels.test(prev))) {
//           primary += 'H'
//           secondary += 'H'

//           index++
//         }

//         index++

//         break
//       'J' =>
//         // Obvious Spanish, `jose`, `San Jacinto`.
//         if (
//           value.slice(index, index + 4) === 'JOSE' ||
//           value.slice(0, 4) === 'SAN '
//         ) {
//           if (
//             value.slice(0, 4) === 'SAN ' ||
//             (index === 0 && characters[index + 4] === ' ')
//           ) {
//             primary += 'H'
//             secondary += 'H'
//           } else {
//             primary += 'J'
//             secondary += 'H'
//           }

//           index++

//           break
//         }

//         if (
//           index === 0
//           // Bug: unreachable (see previous statement).
//           // && value.slice(index, index + 4) !== 'JOSE'.
//         ) {
//           primary += 'J'

//           // Such as `Yankelovich` or `Jankelowicz`.
//           secondary += 'A'
//           // Spanish pron. of such as `bajador`.
//         } else if (
//           !isSlavoGermanic &&
//           (next === 'A' || next === 'O') &&
//           vowels.test(prev)
//         ) {
//           primary += 'J'
//           secondary += 'H'
//         } else if (index === last) {
//           primary += 'J'
//         } else if (
//           prev !== 'S' &&
//           prev !== 'K' &&
//           prev !== 'L' &&
//           !jForJException.test(next)
//         ) {
//           primary += 'J'
//           secondary += 'J'
//           // It could happen.
//         } else if (next === 'J') {
//           index++
//         }

//         index++

//         break
//       'K' =>
//         if (next === 'K') {
//           index++
//         }

//         primary += 'K'
//         secondary += 'K'
//         index++

//         break
//       'L' =>
//         if (next === 'L') {
//           // Spanish such as `cabrillo`, `gallegos`.
//           if (
//             (index === length - 3 &&
//               ((prev === 'A' && nextnext === 'E') ||
//                 (prev === 'I' && (nextnext === 'O' || nextnext === 'A')))) ||
//             (prev === 'A' &&
//               nextnext === 'E' &&
//               (characters[last] === 'A' ||
//                 characters[last] === 'O' ||
//                 alle.test(value.slice(last - 1, length))))
//           ) {
//             primary += 'L'
//             index += 2

//             break
//           }

//           index++
//         }

//         primary += 'L'
//         secondary += 'L'
//         index++

//         break
//       'M' =>
//         if (
//           next === 'M' ||
//           // Such as `dumb`, `thumb`.
//           (prev === 'U' &&
//             next === 'B' &&
//             (index + 1 === last || value.slice(index + 2, index + 4) === 'ER'))
//         ) {
//           index++
//         }

//         index++
//         primary += 'M'
//         secondary += 'M'

//         break
//       'N' =>
//         if (next === 'N') {
//           index++
//         }

//         index++
//         primary += 'N'
//         secondary += 'N'

//         break
//       case 'Ñ':
//         index++
//         primary += 'N'
//         secondary += 'N'

//         break
//       'P' =>
//         if (next === 'H') {
//           primary += 'F'
//           secondary += 'F'
//           index += 2

//           break
//         }

//         // Also account for `campbell` and `raspberry`.
//         subvalue = next

//         if (subvalue === 'P' || subvalue === 'B') {
//           index++
//         }

//         index++

//         primary += 'P'
//         secondary += 'P'

//         break
//       'Q' =>
//         if (next === 'Q') {
//           index++
//         }

//         index++
//         primary += 'K'
//         secondary += 'K'

//         break
//       'R' =>
//         // French such as `Rogier`, but exclude `Hochmeier`.
//         if (
//           index === last &&
//           !isSlavoGermanic &&
//           prev === 'E' &&
//           characters[index - 2] === 'I' &&
//           characters[index - 4] !== 'M' &&
//           (characters[index - 3] !== 'E' && characters[index - 3] !== 'A')
//         ) {
//           secondary += 'R'
//         } else {
//           primary += 'R'
//           secondary += 'R'
//         }

//         if (next === 'R') {
//           index++
//         }

//         index++

//         break
//       'S' =>
//         // Special cases `island`, `isle`, `carlisle`, `carlysle`.
//         if (next === 'L' && (prev === 'I' || prev === 'Y')) {
//           index++

//           break
//         }

//         // Special case `sugar-`.
//         if (index === 0 && value.slice(1, 5) === 'UGAR') {
//           primary += 'X'
//           secondary += 'S'
//           index++

//           break
//         }

//         if (next === 'H') {
//           // Germanic.
//           if (hForS.test(value.slice(index + 1, index + 5))) {
//             primary += 'S'
//             secondary += 'S'
//           } else {
//             primary += 'X'
//             secondary += 'X'
//           }

//           index += 2
//           break
//         }

//         if (
//           next === 'I' &&
//           (nextnext === 'O' || nextnext === 'A')
//           // Bug: Already covered by previous branch
//           // || value.slice(index, index + 4) === 'SIAN'
//         ) {
//           if (isSlavoGermanic) {
//             primary += 'S'
//             secondary += 'S'
//           } else {
//             primary += 'S'
//             secondary += 'X'
//           }

//           index += 3

//           break
//         }

//         // German & Anglicization's, such as `Smith` match `Schmidt`, `snider`
//         // match `Schneider`. Also, -sz- in slavic language although in
//         // hungarian it is pronounced `s`.
//         if (
//           next === 'Z' ||
//           (index === 0 &&
//             (next === 'L' || next === 'M' || next === 'N' || next === 'W'))
//         ) {
//           primary += 'S'
//           secondary += 'X'

//           if (next === 'Z') {
//             index++
//           }

//           index++

//           break
//         }

//         if (next === 'C') {
//           // Schlesinger's rule.
//           if (nextnext === 'H') {
//             subvalue = value.slice(index + 3, index + 5)

//             // Dutch origin, such as `school`, `schooner`.
//             if (dutchSch.test(subvalue)) {
//               // Such as `schermerhorn`, `schenker`.
//               if (subvalue === 'ER' || subvalue === 'EN') {
//                 primary += 'X'
//                 secondary += 'SK'
//               } else {
//                 primary += 'SK'
//                 secondary += 'SK'
//               }

//               index += 3

//               break
//             }

//             if (
//               index === 0 &&
//               !vowels.test(characters[3]) &&
//               characters[3] !== 'W'
//             ) {
//               primary += 'X'
//               secondary += 'S'
//             } else {
//               primary += 'X'
//               secondary += 'X'
//             }

//             index += 3

//             break
//           }

//           if (nextnext === 'I' || nextnext === 'E' || nextnext === 'Y') {
//             primary += 'S'
//             secondary += 'S'
//             index += 3
//             break
//           }

//           primary += 'SK'
//           secondary += 'SK'
//           index += 3

//           break
//         }

//         subvalue = value.slice(index - 2, index)

//         // French such as `resnais`, `artois`.
//         if (index === last && (subvalue === 'AI' || subvalue === 'OI')) {
//           secondary += 'S'
//         } else {
//           primary += 'S'
//           secondary += 'S'
//         }

//         if (
//           next === 'S'
//           // Bug: already taken care of by `German & Anglicization's` above:
//           // || next === 'Z'
//         ) {
//           index++
//         }

//         index++

//         break
//       'T' =>
//         if (next === 'I' && nextnext === 'O' && characters[index + 3] === 'N') {
//           primary += 'X'
//           secondary += 'X'
//           index += 3

//           break
//         }

//         subvalue = value.slice(index + 1, index + 3)

//         if (
//           (next === 'I' && nextnext === 'A') ||
//           (next === 'C' && nextnext === 'H')
//         ) {
//           primary += 'X'
//           secondary += 'X'
//           index += 3

//           break
//         }

//         if (next === 'H' || (next === 'T' && nextnext === 'H')) {
//           // Special case `Thomas`, `Thames` or Germanic.
//           if (
//             isGermanic ||
//             ((nextnext === 'O' || nextnext === 'A') &&
//               characters[index + 3] === 'M')
//           ) {
//             primary += 'T'
//             secondary += 'T'
//           } else {
//             primary += '0'
//             secondary += 'T'
//           }

//           index += 2

//           break
//         }

//         if (next === 'T' || next === 'D') {
//           index++
//         }

//         index++
//         primary += 'T'
//         secondary += 'T'

//         break
//       'V' =>
//         if (next === 'V') {
//           index++
//         }

//         primary += 'F'
//         secondary += 'F'
//         index++

//         break
//       'W' =>
//         // Can also be in middle of word (as already taken care of for initial).
//         if (next === 'R') {
//           primary += 'R'
//           secondary += 'R'
//           index += 2

//           break
//         }

//         if (index === 0) {
//           // `Wasserman` should match `Vasserman`.
//           if (vowels.test(next)) {
//             primary += 'A'
//             secondary += 'F'
//           } else if (next === 'H') {
//             // Need `Uomo` to match `Womo`.
//             primary += 'A'
//             secondary += 'A'
//           }
//         }

//         // `Arnow` should match `Arnoff`.
//         if (
//           ((prev === 'E' || prev === 'O') &&
//             next === 'S' &&
//             nextnext === 'K' &&
//             (characters[index + 3] === 'I' || characters[index + 3] === 'Y')) ||
//           // Maybe a bug? Shouldn't this be general Germanic?
//           value.slice(0, 3) === 'SCH' ||
//           (index === last && vowels.test(prev))
//         ) {
//           secondary += 'F'
//           index++

//           break
//         }

//         // Polish such as `Filipowicz`.
//         if (
//           next === 'I' &&
//           (nextnext === 'C' || nextnext === 'T') &&
//           characters[index + 3] === 'Z'
//         ) {
//           primary += 'TS'
//           secondary += 'FX'
//           index += 4

//           break
//         }

//         index++

//         break
//       'X' =>
//         // French such as `breaux`.
//         if (
//           !(
//             index === last &&
//             // Bug: IAU and EAU also match by AU
//             // (/IAU|EAU/.test(value.slice(index - 3, index))) ||
//             (prev === 'U' &&
//               (characters[index - 2] === 'A' || characters[index - 2] === 'O'))
//           )
//         ) {
//           primary += 'KS'
//           secondary += 'KS'
//         }

//         if (next === 'C' || next === 'X') {
//           index++
//         }

//         index++

//         break
//       'Z' =>
//         // Chinese pinyin such as `Zhao`.
//         if (next === 'H') {
//           primary += 'J'
//           secondary += 'J'
//           index += 2

//           break
//         } else if (
//           (next === 'Z' &&
//             (nextnext === 'A' || nextnext === 'I' || nextnext === 'O')) ||
//           (isSlavoGermanic && index > 0 && prev !== 'T')
//         ) {
//           primary += 'S'
//           secondary += 'TS'
//         } else {
//           primary += 'S'
//           secondary += 'S'
//         }

//         if (next === 'Z') {
//           index++
//         }

//         index++

//         break
//       default:
//         index++
//     }
//   }

//   return [primary, secondary]
// }
