use std::collections::HashMap;

pub fn less_fuel(pos: &str) -> u32 {
    let pos_list: Vec<u16> = pos
        .trim()
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect();
    let mut fuel = 0u32;
    let mut hpos = 2u16;
    // create a hm with most common
    let mut map: HashMap<u16, u32> = HashMap::new();
    pos_list.iter().for_each(|pos| {
        map.entry(*pos).and_modify(|c| *c += 1).or_insert(1);
    });
    hpos = *map.iter().max_by_key(|el| el.1).unwrap().0;
    pos_list.iter().for_each(|pos| {
        fuel += pos.abs_diff(hpos) as u32;
    });
    println!("{hpos}");
    fuel
}

pub fn less_perebor(pos: &str) -> u32 {
    let pos_list: Vec<u16> = pos
        .trim()
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect();
    let mut low_pos: u16 = *pos_list.iter().min().unwrap();
    let mut cur_pos: u16 = *pos_list.iter().min().unwrap();
    let high_pos: u16 = *pos_list.iter().max().unwrap();
    let mut min_fuel: u32 = u32::MAX;
    let mut cur_fuel: u32 = 0;
    while cur_pos < high_pos {
        pos_list.iter().for_each(|pos| {
            cur_fuel += pos.abs_diff(cur_pos) as u32;
        });
        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
            low_pos = cur_pos;
        }
        cur_pos += 1;
        cur_fuel = 0;
    }
    println!("{low_pos}");
    min_fuel
}
pub fn less_perebor_mul(pos: &str) -> u32 {
    let pos_list: Vec<u16> = pos
        .trim()
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect();
    {
        println!(
            "seredne : {}",
            pos_list.iter().map(|n| *n as u32).sum::<u32>() / pos_list.len() as u32
        );
    }
    let mut low_pos: u16 = *pos_list.iter().min().unwrap();
    let mut cur_pos: u16 = *pos_list.iter().min().unwrap();
    let high_pos: u16 = *pos_list.iter().max().unwrap();
    let mut min_fuel: u32 = u32::MAX;
    let mut cur_fuel: u32 = 0;
    while cur_pos < high_pos {
        pos_list.iter().for_each(|pos| {
            let diff = pos.abs_diff(cur_pos) as u32;
            cur_fuel += diff * (diff + 1) / 2;
        });
        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
            low_pos = cur_pos;
        }
        cur_pos += 1;
        cur_fuel = 0;
    }
    println!("{low_pos}");
    min_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7p1_works() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        // let inp = "0,1,1,2,2,2,4,7,14,16";
        let res = less_perebor(inp);
        // let res = less_fuel(inp);
        assert_eq!(res, 37);
    }
    #[test]
    fn d7p2_works() {
        let inp = "16,1,2,0,4,2,7,1,2,14";
        let res = less_perebor_mul(inp);
        // let res = less_fuel(inp);
        // 93 214 037 at 484
        assert_eq!(res, 168_0);
    }
}
