use d2::{go, go_aim};
use std::fs;

fn main() {
    let course = fs::read_to_string("input.txt").unwrap();
    println!("p1: {}", go(course.as_str()));
    println!("p2: {}", go_aim(course.as_str()));
}
