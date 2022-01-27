use crate::frequency_map;
use std::collections::HashMap;

pub fn valid_anagram(s: String, t: String) -> bool {
    let s_map: HashMap<char, i32> = frequency_map::build_frequency_map(s);
    let mut t_map: HashMap<char, i32> = frequency_map::build_frequency_map(t);

    for (letter, count) in s_map.iter() {
        match t_map.get_mut(&letter) {
            Some(freq) => {
                if count.ne(freq) {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::valid_anagram;

    #[test]
    fn test_valid_anagram() {
        let string1: String = "anagram".to_string();
        let string2: String = "aganram".to_string();

        assert_eq!(valid_anagram(string1, string2), true,);
    }

    #[test]
    fn test_invalid_anagram_different_letters() {
        let string1: String = "anagram".to_string();
        let string2: String = "atsushi".to_string();

        assert_eq!(valid_anagram(string1, string2), false,);
    }

    #[test]
    fn test_invalid_anagram_same_letters() {
        let string1: String = "anagram".to_string();
        let string2: String = "angrm".to_string();

        assert_eq!(valid_anagram(string1, string2), false,);
    }
}
