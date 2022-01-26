use std::collections::HashSet;

pub fn contains_duplicates(nums: Vec<i32>) -> bool {
    let mut visited: HashSet<&i32> = HashSet::with_capacity(nums.len());

    for num in nums.iter() {
        if visited.contains(&num) {
            return true;
        } else {
            visited.insert(&num);
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
