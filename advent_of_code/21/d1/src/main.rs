use d1::{calculade_iter, calculate_three};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = calculade_iter(input.as_str());
    let res2 = calculate_three(input.as_str());
    println!("{res}");
    println!("{res2}");
}
