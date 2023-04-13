fn main() {
    let mut s = String::from("Hello");
    let adder = |s: &mut String| s.push_str(" world");
    println!("{s}");
    adder(&mut s);
}
