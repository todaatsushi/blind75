mod three_sum;
mod two_sum;

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!("{:?}", three_sum::three_sum(nums));
    println!("{:?}", two_sum::two_sum(nums));
}
