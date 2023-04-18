use std::io;

struct Temperature(i32);

impl Temperature {
    fn to_celsius(&self) -> i32 {
        ((self.0 - 32) * 5) / 9
    }
    fn to_farenheight(&self) -> i32 {
        ((self.0 * 9) / 5) + 32
    }
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Can't read line");
    
    input.trim().to_string()
}

fn str_to_int(string: &str) -> i32 {
    string.trim().parse().expect("Enter a number")
}

fn main() {
    println!("This is a temperature convertor\nYou can convert between celsius and farenheight");
    println!("Please enter a temperarure as 25c or 50f");
    
    let input = read_input();
    let (value, scale) = input.split_at(input.len() - 1);
    let temperature = Temperature(str_to_int(value));

    match scale {
        "c" | "C" => println!("It's {} in farengeits", temperature.to_farenheight()),
        "f" | "F" => println!("It's {} in celsius", temperature.to_celsius()),
        _ => println!("Can't recognize scale"),
    };
}

