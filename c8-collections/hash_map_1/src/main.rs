fn main() {
    let res1 = array_median_mode(vec![3, 4, 3, 3, 3, 9, 2, 2, 2, 2]);
    println!("{:?}", res1);
    let res2 = pig_latin("hash maps will provide a large amount of functionality necessary in programs when you need to store");
    println!("{:?}", res2);
}

fn pig_latin(s: &str) -> String {
    let mut res = String::new();

    for word in s.split_whitespace() {
        let chars: Vec<_> = word.chars().collect();
        let new_word: String;

        if "aeiouy".contains(chars[0]) {
            new_word = format!("{}-hey ", word);
        } else {
            new_word = format!("{}-{}ey ", chars[1..].iter().collect::<String>(), chars[0]);
        }
        res.push_str(new_word.as_str());
    }
    res.to_string()
}

fn array_median_mode(mut a: Vec<i32>) -> (i32, i32) {
    use std::collections::HashMap;
    a.sort();
    let median = a[a.len() / 2];

    let mut map = HashMap::new();
    let mut mode: i32 = -1;
    let mut max: i32 = 0;

    for val in a {
        let count = map.entry(val).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            mode = val;
        }
    }

    (median, mode)
}
