use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
#[macro_export]
macro_rules! strings_iter {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec.into_iter()
        }
    };
}
fn main() {
    let vec = Vec::from([1, 2]);
    println!("{:?}", vec);
    let mv = myvec![(3, 4, 5), (6, 7, 8)];
    println!("{:?}", mv);
    let rv = vec![11, 11, 22, 33, 44];
    println!("{:?}", rv);
    let strings = strings_iter!("lol", "kek", "pus", "puk", "kek");
    let hs: HashSet<String> = HashSet::from_iter(strings);
    println!("{:?}", hs);
}
