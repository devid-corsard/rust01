use std::io;
// smarter code with chatGPT

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

fn cel_to_far(temp: i32) -> i32 {
    ((temp * 9) / 5) + 32
}

fn far_to_cel(temp: i32) -> i32 { 
    ((temp - 32) * 5) / 9
}

fn main() {
    println!("This is a temperature convertor\nYou can convert between celsius and farenheight");
    println!("Please enter a temperarure as 25c or 50f");
    
    let input = read_input();
    let (value, scale) = input.split_at(input.len() - 1);
    let value = str_to_int(value);

    match scale {
        "c" | "C" => println!("It's {} in farengeits", cel_to_far(value)),
        "f" | "F" => println!("It's {} in celsius", far_to_cel(value)),
        _ => println!("Can't recognize scale"),
    };
}

