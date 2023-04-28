struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let Some(x) = Some(10) else {todo!()};
    println!("{x}");

    let p = Point { x: 4, y: 5 };

    match p {
        Point { x: k @ 0..=5, y } if k + y != 0 => println!("COOL"),
        _ => println!("not COOL"),
    }
}
