mod max_subarray;

fn main() {
    let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{}", max_subarray::get_max_subarray(nums));
}
