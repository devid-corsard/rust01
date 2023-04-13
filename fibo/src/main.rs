fn main() {
    let a = Box::new("lol");
    let b = Box::new(55);
    let c = Box::new(88.55);
    let mut vec: Vec<Box<T>> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    println!("{:?}", vec);
}

fn fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}
// fib100  354224848179261915075
// maxu64  18446744073709551615
// maxu128 340282366920938463463374607431768211455
