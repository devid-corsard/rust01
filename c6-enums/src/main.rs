fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}
fn main() {
  println!("{}", make_separator("Wait. What?"));
  println!("{}", make_separator(""));
}
