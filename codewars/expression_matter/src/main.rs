use std::thread;

fn main() {
    let handles: Vec<_> = (0..10_000).map(|_| {
        thread::spawn(|| {
            let mut x = 0u64;
            for _ in 0..1_000_000_000_000u64 {
                x += 1
            }
            x
        })
    }).collect();

    let mut results = Vec::new();
    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }

    println!("Results: {:?}", results);
}

