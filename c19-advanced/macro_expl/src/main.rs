use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

struct Cakes;

impl HelloMacro for Cakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Cakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
    Cakes::hello_macro();
}
