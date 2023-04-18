fn double(value: &i32) -> i32 {
    value * 2
}

fn main() {
    let x = 5;
    let y = &double(&x);
    println!("{},{}",x,y);
}
