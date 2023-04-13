fn main() {
    let word = "goodbye";

    let count = word.chars().count();
    assert_eq!(7, count);
    let count = word.len();
    assert_eq!(7, count);
}
