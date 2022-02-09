pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    vec![nums]
}

// #[cfg(test)]
// mod tests {
//     use super::three_sum;

//     #[test]
//     fn test_finds_correct_combos_in_input() {
//         let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
//         let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

//         assert_eq!(expected, three_sum(nums));
//     }

//     #[test]
//     fn test_finds_no_combos_in_impossible_input() {
//         let nums: Vec<i32> = vec![-1];
//         let expected: Vec<Vec<i32>> = vec![];

//         assert_eq!(expected, three_sum(nums));
//     }

//     #[test]
//     fn test_finds_no_combos_in_no_input() {
//         let nums: Vec<i32> = vec![];
//         let expected: Vec<Vec<i32>> = vec![];

//         assert_eq!(expected, three_sum(nums));
//     }
// }
