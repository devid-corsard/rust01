fn fake_bin(s: &str) -> String {
    //let mut res = String::new();

    //    for char in s.chars() {
    //        let digit: u32 = char.to_string().parse().unwrap();
    //        if digit < 5 {
    //            res.push_str("0")
    //        } else {
    //            res.push_str("1")
    //        }
    //    }
    //    for char in s.chars() {
    //        if "01234".contains(char) {
    //            res.push_str("0")
    //        } else {
    //            res.push_str("1")
    //        }
    //    }
    //let mut str2 = s.clone();
    s.chars()
        .map(|c| if "01234".contains(c) { "0" } else { "1" })
        .collect::<String>()
    //    str2.to_string()
}
fn main() {
    println!("{}", fake_bin("45385593107843568"));
    println!("{}", fake_bin("509321967506747"));
    println!("{}", fake_bin("366058562030849490134388085"));
    println!("{}", fake_bin("15889923"));
    println!("{}", fake_bin("800857237867"));
}
