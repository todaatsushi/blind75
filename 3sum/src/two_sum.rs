use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    let mut num: i32;

    for index in 0..nums.len() {
        num = nums[index];
        match complements.get_mut(&(target - num)) {
            Some(match_index) => return vec![1i32, num, nums.get(&match_index)],
            None => complements.insert(target - num, index),
        }
    }
    vec![0i32]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_finds_correct_combos_in_input() {
        let nums: Vec<i32> = vec![5i32, -1, 0, 1, 2, -1, -4];
        let target: i32 = -2;
        let expected: Vec<i32> = vec![1i32, 0, 4];

        assert_eq!(expected, two_sum(nums));
    }

    #[test]
    fn test_finds_no_combos_in_impossible_input() {
        let nums: Vec<i32> = vec![0i32, -1];
        let expected: Vec<i32> = vec![];

        assert_eq!(expected, two_sum(nums));
    }

    #[test]
    fn test_finds_no_combos_in_no_input() {
        let nums: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];

        assert_eq!(expected, two_sum(numc));
    }
}
