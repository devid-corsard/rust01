use std::fs;

use d5::{danger_points, danger_points_all};
fn main() {
    let coords = fs::read_to_string("input.txt").unwrap();
    println!("p1 {}", danger_points(coords.as_str()));
    println!("p2 {}", danger_points_all(coords.as_str()));
}
