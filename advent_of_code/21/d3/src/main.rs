use d3::{o2_co2, power_cons};
use std::fs;
fn main() {
    let diag = fs::read_to_string("input.txt").unwrap();
    println!("{}", power_cons(diag.as_str()));
    println!("{}", o2_co2(diag.as_str()));
}
