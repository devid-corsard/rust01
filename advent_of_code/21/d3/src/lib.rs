pub fn power_cons(diag: &str) -> u32 {
    let diag: Vec<Vec<char>> = diag.lines().map(|bstr| bstr.chars().collect()).collect();
    let height = diag.len();
    let width = diag[0].len();

    let mut gamma = String::new();
    let mut epsilon = String::new();
    let mut arr: Vec<usize> = Vec::with_capacity(width);
    let mut x = 0usize;
    let mut y = 0usize;
    while x < width {
        y = 0;
        arr.push(0);
        while y < height {
            if diag[y][x] == '1' {
                arr[x] += 1;
            }
            y += 1;
        }
        x += 1;
    }
    for sum in arr {
        if sum * 2 > height {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    println!("{} {}", gamma, epsilon);
    let res = u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap();
    res
}
pub fn o2_co2(diag: &str) -> u32 {
    let diag: Vec<Vec<char>> = diag.lines().map(|bstr| bstr.chars().collect()).collect();
    let mut diag_o2 = diag.clone();
    let mut diag_co2 = diag.clone();
    let mut height = diag.len();
    let width = diag[0].len();

    let mut arr: Vec<usize> = Vec::with_capacity(width);
    let mut x = 0usize;
    let mut y = 0usize;
    while x < width {
        y = 0;
        arr.push(0);
        while y < height {
            if diag_o2[y][x] == '1' {
                arr[x] += 1;
            }
            y += 1;
        }
        if arr[x] * 2 >= height {
            diag_o2 = diag_o2.into_iter().filter(|n| n[x] == '1').collect();
        } else {
            diag_o2 = diag_o2.into_iter().filter(|n| n[x] == '0').collect();
        }
        height = diag_o2.len();
        if height == 1 {
            break;
        }
        x += 1;
    }
    x = 0;
    arr.clear();
    height = diag.len();
    while x < width {
        y = 0;
        arr.push(0);
        while y < height {
            if diag_co2[y][x] == '1' {
                arr[x] += 1;
            }
            y += 1;
        }
        if arr[x] * 2 >= height {
            diag_co2 = diag_co2.into_iter().filter(|n| n[x] == '0').collect();
        } else {
            diag_co2 = diag_co2.into_iter().filter(|n| n[x] == '1').collect();
        }
        height = diag_co2.len();
        if height == 1 {
            break;
        }
        x += 1;
    }
    let o2_str = diag_o2[0].iter().collect::<String>();
    let co2_str = diag_co2[0].iter().collect::<String>();
    // for sum in arr {
    //     if sum * 2 > height {
    //         gamma.push('1');
    //         epsilon.push('0');
    //     } else {
    //         gamma.push('0');
    //         epsilon.push('1');
    //     }
    // }
    println!("{} {}", o2_str, co2_str);
    let res = u32::from_str_radix(&o2_str, 2).unwrap() * u32::from_str_radix(&co2_str, 2).unwrap();
    res
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_works() {
        let diag = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let result = power_cons(diag);
        assert_eq!(result, 198);
    }
    #[test]
    fn p2_works() {
        let diag = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let result = o2_co2(diag);
        assert_eq!(result, 230);
    }
}
