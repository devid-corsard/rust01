pub fn lowest_points(map: &str) -> u32 {
    let map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let height = map.len();
    let width = map[0].len();
    let mut x: usize;
    let mut y: usize;
    let mut low_points: Vec<u32> = Vec::new();

    y = 0;
    while y < height {
        x = 0;
        while x < width {
            if map[y][x] == 9 {
                x += 1;
                continue;
            }
            let neighbours = get_neighbours(x, y, &map);
            if neighbours.iter().all(|&n| n > map[y][x]) {
                low_points.push(map[y][x] as u32 + 1);
            }
            x += 1;
        }

        y += 1;
    }
    low_points.iter().sum()
    // 0
}

pub fn basins(map: &str) -> u32 {
    let map = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let height = map.len();
    let width = map[0].len();
    let mut x: usize;
    let mut y: usize;
    let mut low_points: Vec<u32> = Vec::new();
    let mut low_points_coords: Vec<(usize, usize)> = Vec::new();

    y = 0;
    while y < height {
        x = 0;
        while x < width {
            if map[y][x] == 9 {
                x += 1;
                continue;
            }
            let neighbours = get_neighbours(x, y, &map);
            if neighbours.iter().all(|&n| n > map[y][x]) {
                low_points.push(map[y][x] as u32 + 1);
                low_points_coords.push((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    let mut basin_sizes: Vec<u32> = Vec::new();
    low_points_coords.iter().for_each(|&(x, y)| {
        let mut to_check: Vec<(Vec<(usize, usize)>, (usize, usize))> =
            Vec::from([(get_neighbours_coords(x, y, &map), (x, y))]);
        let mut checked: Vec<(usize, usize)> = Vec::new();
        let mut basin_coords = Vec::from([(x, y)]);

        while let Some((vec_coords, cmp_coord)) = to_check.pop() {
            let (x, y) = cmp_coord;
            checked.push(cmp_coord);
            vec_coords.iter().for_each(|&(nx, ny)| {
                if map[ny][nx] == map[y][x] + 1 && map[ny][nx] != 9 {
                    if !basin_coords.contains(&(nx, ny)) {
                        basin_coords.push((nx, ny));
                    }
                    let neig_coords = get_neighbours_coords(nx, ny, &map)
                        .into_iter()
                        .filter(|coord| !checked.contains(coord))
                        .collect();
                    to_check.push((neig_coords, (nx, ny)));
                }
            });
        }
        println!("bas size: {}", basin_coords.len());
        basin_sizes.push(basin_coords.len() as u32);
    });
    println!("sizes: {:?}", basin_sizes);
    basin_sizes.sort();
    println!("sizes: {:?}", basin_sizes);
    basin_sizes.iter().rev().take(3).product()
    // 0
}

fn get_neighbours_coords(x: usize, y: usize, map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(4);
    let (x, y) = (x as i32, y as i32);
    let (l, r, t, b) = ((y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x));
    [l, r, t, b].iter().for_each(|&(y, x)| {
        if y >= 0 && y < height && x >= 0 && x < width {
            neighbours.push((x as usize, y as usize));
        }
    });
    neighbours
}
fn get_neighbours(x: usize, y: usize, map: &Vec<Vec<u8>>) -> Vec<u8> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut neighbours: Vec<u8> = Vec::with_capacity(4);
    let (x, y) = (x as i32, y as i32);
    let (l, r, t, b) = ((y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x));
    [l, r, t, b].iter().for_each(|&(y, x)| {
        if y >= 0 && y < height && x >= 0 && x < width {
            neighbours.push(map[y as usize][x as usize]);
        }
    });
    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1_works() {
        let inp = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let res = lowest_points(inp);
        assert_eq!(res, 15);
    }
    #[test]
    fn d9p2_works() {
        let inp = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let res = basins(inp);
        assert_eq!(res, 1134);
    }
}
