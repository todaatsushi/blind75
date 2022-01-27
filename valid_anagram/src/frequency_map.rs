use std::collections::HashMap;

pub fn build_frequency_map(input: String) -> HashMap<char, i32> {
    let mut frequency_map: HashMap<char, i32> = HashMap::new();

    for letter in input.chars() {
        *frequency_map.entry(letter).or_insert(0) += 1;
    }
    return frequency_map;
}

#[cfg(test)]
mod tests {
    use super::build_frequency_map;
    use super::HashMap;

    #[test]
    fn test_build_frequency_map() {
        let name: String = "atsushi".to_string();
        let mut expected: HashMap<char, i32> = HashMap::new();

        expected.insert('a', 1);
        expected.insert('t', 1);
        expected.insert('s', 2);
        expected.insert('u', 1);
        expected.insert('h', 1);
        expected.insert('i', 1);

        assert_eq!(expected, build_frequency_map(name));
    }
}
