mod check_valid;
mod frequency_map;

fn main() {
    let rat: String = "rat".to_string();
    let car: String = "car".to_string();

    println!("{:?}", check_valid::valid_anagram(rat, car));
}
