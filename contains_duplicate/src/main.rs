mod contains_duplicate;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(1);
    nums.push(2);

    println!("{}", contains_duplicate::contains_duplicates(nums));
}
