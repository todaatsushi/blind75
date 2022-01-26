use std::collections::HashMap;

pub fn contains_duplicates(nums: Vec<i32>) -> bool {
    let mut visited: HashMap<&i32, bool> = HashMap::new();

    for num in nums.iter() {
        if visited.contains_key(&num) {
            return true;
        } else {
            visited.insert(&num, false);
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::contains_duplicates;

    #[test]
    fn test_contains_duplicate() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let expected: bool = true;

        assert_eq!(contains_duplicates(nums), expected);
    }

    #[test]
    fn test_doesnt_contain_duplicate() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let expected: bool = false;

        assert_eq!(contains_duplicates(nums), expected);
    }
}
