// use std::fs;
use std::{thread, time};

pub fn calculate(map: &str) -> String {
    let mut map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let mut moves_possible = true;
    let mut counter = 0;
    let mut y;
    let mut x;
    let height = map.len();
    let width = map[0].len();
    let mut moves: Vec<(usize, usize)> = Vec::with_capacity(height);

    while moves_possible {
        moves_possible = false;
        counter += 1;
        y = 0;
        while y < height {
            x = 0;
            while x < width {
                let next_x = if x + 1 < width { x + 1 } else { 0 };
                if map[y][x] == '>' && map[y][next_x] == '.' {
                    moves.push((x, next_x));
                    x += 1;
                    moves_possible = true;
                }
                x += 1;
            }
            while let Some((a, b)) = moves.pop() {
                map[y][a] = '.';
                map[y][b] = '>';
            }
            y += 1;
        }
        x = 0;
        while x < width {
            y = 0;
            while y < height {
                let next_y = if y + 1 < height { y + 1 } else { 0 };
                if map[y][x] == 'v' && map[next_y][x] == '.' {
                    moves.push((y, next_y));
                    y += 1;
                    moves_possible = true;
                }
                y += 1;
            }
            while let Some((a, b)) = moves.pop() {
                map[a][x] = '.';
                map[b][x] = 'v';
            }
            x += 1;
        }
        // thread::sleep(time::Duration::from_millis(100));
        // println!("After {counter} step:");
        // println!(
        //     "{}",
        //     map.iter()
        //         .map(|line| format!("{}{}", line.iter().collect::<String>(), "\n"))
        //         .collect::<String>()
        // );
    }
    println!("Stop moving after {counter} steps");
    let mut res = map
        .iter()
        .map(|line| format!("{}{}", line.iter().collect::<String>(), "\n"))
        .collect::<String>();
    res.pop();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finest() {
        let init_map = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let res = "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..";
        assert_eq!(calculate(init_map), res);
    }
}
