use error::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

type Vals = Vec<Vec<String>>;

pub fn from_file(path: Option<&str>) -> Result<HashMap<String, Vals>, Error> {
    let default_path = "res/cmudict.dict";
    let dict_string: String;

    if let Some(p) = path {
        dict_string = fs::read_to_string(p)?;
    } else {
        dict_string = fs::read_to_string(default_path)?;
    }

    let cursor = io::Cursor::new(dict_string);
    let lines = cursor.lines().collect::<Result<Vec<_>, _>>()?;

    let mut dict_map: HashMap<String, Vals> = HashMap::new();

    for line in lines {
        let entry = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if let Some((h, t)) = entry.split_first() {
            if let Some(key) = h.split('(').collect::<Vec<&str>>().first() {
                match dict_map.get_mut(*key) {
                    Some(v) => {
                        v.push(t.to_vec());
                    }
                    None => {
                        dict_map.insert(key.to_string(), vec![t.to_vec()]);
                    }
                }
            }
        }
    }

    Ok(dict_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _from_file() {
        let dict = from_file(None);
        assert!(dict.is_ok());

        fs::write("dict.out", format!("{:#?}", dict)).expect("Unable to write file");

        assert!(true);
    }
}
