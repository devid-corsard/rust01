fn main() {
    let a = String::from("Hello!");
    println!("{}", a);
    let a = &a;
    // println!("{}", b);
    let a = &&&&a;
    // println!("{}", c);
    let a: &str = a;
    println!("{}", a);
}
