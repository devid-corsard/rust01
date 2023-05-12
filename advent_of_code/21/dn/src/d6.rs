pub fn lanternfish_life(start: &str, day: usize) -> usize {
    let mut map = start.trim().split(',').fold([0; 9], |mut map, n| {
        map[n.parse::<usize>().unwrap()] += 1;
        map
    });

    (0..day).for_each(|day| {
        let pregnant = map[0];
        map[0] = map[1];
        map[1] = map[2];
        map[2] = map[3];
        map[3] = map[4];
        map[4] = map[5];
        map[5] = map[6];
        map[6] = map[7];
        map[7] = map[8];
        map[8] = pregnant;
        map[6] += map[8];
    });

    map.iter().sum::<usize>()
}

pub fn lanternfish_life_opympiad(start: &str) -> usize {
    let mut map = start.trim().split(',').fold([0; 9], |mut map, n| {
        map[n.parse::<usize>().unwrap()] += 1;
        map
    });

    (0..100).for_each(|day| {
        map[(day + 7) % 9] += map[day % 9];
        println!("{:?}", map);
    });

    println!("{}", map.iter().sum::<usize>());
    map.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6_p1_works() {
        let input = "3,4,3,1,2";
        let result = lanternfish_life(input, 80);
        assert_eq!(result, 5934);
    }
    #[test]
    fn d6_p2_works() {
        let input = "3,4,3,1,2";
        let result = lanternfish_life(input, 256);
        assert_eq!(result, 26984457539);
    }
}
