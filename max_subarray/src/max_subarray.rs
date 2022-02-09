use std::cmp;

pub fn get_max_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut sub_total: i32 = nums[0];
    let mut prefix: i32 = 0;

    for num in nums.iter() {
        prefix = cmp::max(0, prefix);
        prefix += num;

        sub_total = cmp::max(sub_total, prefix);
    }

    sub_total
}

#[cfg(test)]
mod tests {
    use super::get_max_subarray;

    #[test]
    fn test_gets_correct_total_with_mixed_nums() {
        let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let expected: i32 = 6;

        assert_eq!(expected, get_max_subarray(nums));
    }

    #[test]
    fn test_gets_correct_total_with_positive_nums() {
        let nums: Vec<i32> = vec![2, 1, 3, 4];
        let expected: i32 = 10;

        assert_eq!(expected, get_max_subarray(nums));
    }

    #[test]
    fn test_gets_correct_total_with_negative_nums() {
        let nums: Vec<i32> = vec![-2, -1, -3, -4, -1, -2, -1, -5, -4];
        let expected: i32 = -1;

        assert_eq!(expected, get_max_subarray(nums));
    }
}
