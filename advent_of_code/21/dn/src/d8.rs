use std::collections::{HashMap, HashSet};

pub fn count_uniq(inp: &str) -> u32 {
    let inp: u32 = inp
        .lines()
        .map(|l| {
            let l = l.split(" | ").collect::<Vec<&str>>()[1];
            l.split_whitespace()
                .filter(|sig| [2, 3, 4, 7].contains(&sig.len()))
                .count() as u32
        })
        .sum();
    inp
    // 0
}
// abcdefg > [t,t,t,t,t,t,t]
pub fn decoder(inp: &str) -> u32 {
    // type Abracadabra = HashSet<char>;
    let res: u32 = inp
        .lines()
        .map(|line| {
            let line = line
                .split(" | ")
                .map(|half| {
                    half.split_whitespace()
                        .map(|word| {
                            let mut word = word.chars().collect::<Vec<char>>();
                            word.sort();
                            word.iter().collect::<String>()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>();

            let mut map: HashMap<String, char> = HashMap::with_capacity(10);
            let mut mapinv: HashMap<char, HashSet<char>> = HashMap::with_capacity(10);
            // let mut mapinv: HashMap<char, String> = HashMap::with_capacity(10);

            line[0].iter().for_each(|codnum| {
                let res = match codnum.len() {
                    2 => Some('1'),
                    3 => Some('7'),
                    4 => Some('4'),
                    7 => Some('8'),
                    _ => None,
                };
                if let Some(val) = res {
                    map.insert(codnum.to_string(), val);
                    mapinv.insert(val, codnum.chars().collect::<HashSet<char>>());
                    // mapinv.insert(val, codnum.to_string());
                };
            });

            line[0].iter().for_each(|codnum| {
                let codhs = codnum.chars().collect::<HashSet<char>>();

                let res = match codnum.len() {
                    5 if codhs.is_superset(&mapinv[&'1']) => Some('3'),
                    6 if codhs.is_superset(&mapinv[&'7']) => {
                        if codhs.is_superset(&mapinv[&'4']) {
                            Some('9')
                        } else {
                            Some('0')
                        }
                    }
                    6 => Some('6'),
                    _ => None,
                };
                if let Some(val) = res {
                    map.insert(codnum.to_string(), val);
                    mapinv.insert(val, codhs);
                    // mapinv.insert(val, codnum.to_string());
                };
            });

            line[0].iter().for_each(|codnum| {
                if codnum.len() != 5 {
                    return;
                }
                let codhs = codnum.chars().collect::<HashSet<char>>();

                let res = if mapinv[&'6'].is_superset(&codhs) {
                    Some('5')
                } else if !codhs.is_superset(&mapinv[&'1']) {
                    Some('2')
                } else {
                    None
                };

                if let Some(val) = res {
                    map.insert(codnum.to_string(), val);
                    mapinv.insert(val, codhs);
                    // mapinv.insert(val, codnum.to_string());
                };
            });
            println!("{:?}", map);
            String::from_iter(
                line[1]
                    .iter()
                    .inspect(|suka| println!("ne mogu naiti: {}", suka))
                    .map(|abracadabra| map[&abracadabra[..]]),
            )
            .parse::<u32>()
            .unwrap()

            // .collect::<Vec<Vec<String>>>();
            // println!("{:?}", line);
            //
            // 0
            // l.split_whitespace()
            //     .filter(|sig| [2, 3, 4, 7].contains(&sig.len()))
            //     .count() as u32
        })
        .sum();
    res
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1_works() {
        let inp =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let res = count_uniq(inp);
        assert_eq!(res, 26);
    }
    #[test]
    // #[ignore]
    fn d8p2_works() {
        let inp =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let res = decoder(inp);
        assert_eq!(res, 61_229);
    }
}
