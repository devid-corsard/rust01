//fn swap_vars(x: &String, y: &String) -> (String, String) {
//    (y.to_string(), x.to_string()) 
//}

fn main() {
    let a = String::from("Hello");
    let b = String::from("World");
    let (a, b) = (b, a);
//  let (a, b) = swap_vars(&a, &b);
    println!("{},{}", a, b);
}
