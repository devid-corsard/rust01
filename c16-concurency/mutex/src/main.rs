use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("num is: {num}");
        });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *counter.lock().unwrap());
}
