use crate::pronounciation::double_metaphone;

pub fn rhymes(a: &str, b: &str) -> bool {
    let a_phonetic = double_metaphone(a);
    let b_phonetic = double_metaphone(b);

    println!("{:?}", a_phonetic);
    println!("{:?}", b_phonetic);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_yellow() {
        assert!(!rhymes("here", "near"));
    }
}
