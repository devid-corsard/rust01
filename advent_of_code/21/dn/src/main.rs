mod d6;
mod d7;
mod d8;
mod d9;
mod d9_genius;

fn main() {
    /*
    let input6 = include_str!("../input6.txt");
    println!("p1 {}", d6::lanternfish_life(input6.as_str(), 80));
    println!("p2 {}", d6::lanternfish_life(input6.as_str(), 256));

    let input7 = include_str!("../input7.txt");
    println!("d7p1 {}", d7::less_perebor(input7));
    println!("d7p2 {}", d7::less_perebor_mul(input7));

    let input8 = include_str!("../input8.txt");
    println!("d8p1 {}", d8::count_uniq(input8));
    println!("d8p2 {}", d8::decoder(input8));
    */
    let input9 = include_str!("../input9.txt");
    println!("d9p1 {}", d9::lowest_points(input9));
    println!("d9p2 {}", d9_genius::answer());
}
