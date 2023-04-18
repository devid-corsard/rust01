fn solution(s: &str, a: Vec<usize>) -> String {
    let mut res = String::from("");
    let mut next: usize = 0;

    for _ in 0..a.len() {
        res.push(s.chars().nth(next).unwrap());
        next = a[next];
        if next == 0 {
            break;
        }
    }
    res
}
fn main() {
    println!("{}", solution("cdeo", vec![3, 2, 0, 1]));
    println!("{}", solution("cdeenetpi", vec![5, 2, 0, 1, 6, 4, 8, 3, 7]));
    println!("{}", solution("bytdag", vec![4, 3, 0, 1, 2, 5]));
}
