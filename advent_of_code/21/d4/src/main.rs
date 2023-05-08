use d4::bingo;
use d4::bingo_last;
use std::fs;
fn main() {
    let game = fs::read_to_string("input.txt").unwrap();
    println!("p1 {}", bingo(game.as_str()));
    println!("p2 {}", bingo_last(game.as_str()));
}
