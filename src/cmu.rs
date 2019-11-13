#![feature(advanced_slice_patterns, slice_patterns)]
use error::Error;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

type Vals = Vec<Vec<String>>;

pub fn from_file(path: &str) -> Result<HashMap<String, Vals>, Error> {
    let dict_string = fs::read_to_string(path)?;
    let cursor = io::Cursor::new(dict_string);
    let lines = cursor.lines().collect::<Result<Vec<_>, _>>()?;

    let mut dict_map: HashMap<String, Vals> = HashMap::new();

    for line in lines {
        let entry = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if let Some((h, t)) = entry.split_first() {
            match dict_map.get_mut(&h.clone()) {
                Some(v) => {
                    v.push(t.to_vec());
                }
                None => {
                    dict_map.insert(h.to_string(), vec![t.to_vec()]);
                }
            }
        }
    }

    Ok(dict_map)
}
