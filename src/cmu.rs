extern crate pest;

use error::Error;
use metaphone::{Rule, Word};
use pest::Parser;
use reqwest;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub struct CmuDict {
    dict: HashMap<String, Vec<Vec<String>>>,
}

impl CmuDict {
    pub fn new(path: &str) -> Result<CmuDict, Error> {
        match from_json_file(&Path::new(path)) {
            Ok(d) => Ok(CmuDict { dict: d }),
            Err(e) => Err(e),
        }
    }

    /// CMUdict phonetic encoding.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmu.dict").unwrap();
    /// assert_eq!(
    ///     cmudict.encoding("permeability"),
    ///     Ok(Some(vec![vec![
    ///         "P".to_string(),
    ///         "ER0".to_string(),
    ///         "M".to_string(),
    ///         "IY2".to_string(),
    ///         "AH0".to_string(),
    ///         "B".to_string(),
    ///         "IH1".to_string(),
    ///         "L".to_string(),
    ///         "IH0".to_string(),
    ///         "T".to_string(),
    ///         "IY0".to_string()
    ///     ]]))
    /// );
    ///
    /// assert_eq!(
    ///     cmudict.encoding("unearthed"),
    ///     Ok(Some(vec![vec![
    ///         "AH0".to_string(),
    ///         "N".to_string(),
    ///         "ER1".to_string(),
    ///         "TH".to_string(),
    ///         "T".to_string()
    ///     ]]))
    /// );
    /// ```
    pub fn encoding(&self, w: &str) -> Result<Option<Vec<Vec<String>>>, Error> {
        Ok(self.dict.get(w).map(|v| v.to_vec()))
    }

    /// Use CMUdict phonetic encoding to determine if two words rhyme.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmu.dict").unwrap();
    /// // Does rhyme
    /// assert!(cmudict.rhyme("hissed", "mist").unwrap());
    /// assert!(cmudict.rhyme("tryst", "wrist").unwrap());
    ///
    /// // Does not rhyme
    /// assert!(!cmudict.rhyme("red", "Edmund").unwrap());
    /// assert!(!cmudict.rhyme("comfy", "chair").unwrap());
    /// ```
    pub fn rhyme(&self, a: &str, b: &str) -> Result<bool, Error> {
        if let (Some(phones_a), Some(phones_b)) = (
            self.dict.get(a.to_string().to_lowercase().trim()),
            self.dict.get(b.to_string().to_lowercase().trim()),
        ) {
            return Ok(eval_rhyme(phones_a, phones_b));
        }

        Ok(false)
    }

    /// Use CMUdict phonetic encoding to determine if two words alliterate.
    ///
    /// ```rust
    /// extern crate ttaw;
    /// use ttaw::cmu::CmuDict;
    /// let cmudict = CmuDict::new("cmu.dict").unwrap();
    // // Does alliterate
    /// assert!(cmudict.alliteration("bouncing", "bears").unwrap());
    /// assert!(cmudict.alliteration("snappy", "snails").unwrap());
    ///
    /// // Does not alliterate
    /// assert!(!cmudict.alliteration("brown", "fox").unwrap());
    /// assert!(!cmudict.alliteration("lazy", "dog").unwrap());
    /// ```
    pub fn alliteration(&self, a: &str, b: &str) -> Result<bool, Error> {
        if Word::parse(Rule::vowel_first, a.get(..1).unwrap_or_default()).is_ok() {
            return Ok(false);
        }

        if Word::parse(Rule::vowel_first, b.get(..1).unwrap_or_default()).is_ok() {
            return Ok(false);
        }

        if let (Some(phones_a), Some(phones_b)) = (
            self.dict.get(a.to_string().to_lowercase().trim()),
            self.dict.get(b.to_string().to_lowercase().trim()),
        ) {
            return Ok(eval_alliteration(phones_a, phones_b));
        }

        Ok(false)
    }
}

fn rhyming_part(phones: &[String]) -> Option<Vec<String>> {
    for (i, s) in phones.iter().rev().enumerate() {
        if let Some(num) = s.chars().collect::<Vec<char>>().last() {
            if *num == '1' || *num == '2' {
                return phones.get(phones.len() - 1 - i..).map(|v| v.to_vec());
            }
        }
    }

    None
}

fn eval_rhyme(phones_a: &[Vec<String>], phones_b: &[Vec<String>]) -> bool {
    for a in phones_a {
        for b in phones_b {
            if rhyming_part(a) == rhyming_part(b) {
                return true;
            }
        }
    }

    false
}

fn eval_alliteration(phones_a: &[Vec<String>], phones_b: &[Vec<String>]) -> bool {
    for a in phones_a {
        for b in phones_b {
            if let (Some(a), Some(b)) = (a.first(), b.first()) {
                return a == b;
            }
        }
    }

    false
}

fn from_json_file(path: &Path) -> Result<HashMap<String, Vec<Vec<String>>>, Error> {
    let dict_json: String;

    if !path.exists() {
        // regenerate if the file isn't there
        if path.is_dir() {
            download_and_serialize(&path.join("cmudict.json"))?;
        } else {
            download_and_serialize(&path)?;
        }
    }

    dict_json = fs::read_to_string(path)?;
    let dict: HashMap<String, Vec<Vec<String>>> = serde_json::from_str(&dict_json)?;
    Ok(dict)
}

pub fn download_and_serialize(path: &Path) -> Result<(), Error> {
    let dict_string =
        reqwest::get("https://raw.githubusercontent.com/cmusphinx/cmudict/master/cmudict.dict")?
            .text()?;

    let cursor = io::Cursor::new(dict_string);
    let lines = cursor.lines().collect::<Result<Vec<_>, _>>()?;

    let mut dict: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for line in lines {
        let entry = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if let Some((h, t)) = entry.split_first() {
            if let Some(key) = h.split('(').collect::<Vec<&str>>().first() {
                match dict.get_mut(*key) {
                    Some(v) => {
                        v.push(t.to_vec());
                    }
                    None => {
                        dict.insert(key.to_string(), vec![t.to_vec()]);
                    }
                }
            }
        }
    }

    let serialized = serde_json::to_string(&dict)?;
    fs::write(path, serialized)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_and_serialze() {
        let dir = tempfile::tempdir().unwrap();
        let fpath = dir.path().join("serialized");
        let dict = download_and_serialize(&fpath);
        assert!(dict.is_ok());
    }

    #[test]
    fn test_from_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let fpath = dir.path().join("serialized");
        let dict = from_json_file(&fpath);
        assert!(dict.is_ok());
    }
}
