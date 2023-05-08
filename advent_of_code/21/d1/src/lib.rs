pub fn calculade(s: &str) -> u32 {
    let data = s.lines().map(|el| el.parse::<u32>().unwrap());
    let mut prev = u32::MAX;
    let mut counter = 0;
    for dist in data {
        if dist > prev {
            counter += 1;
        }
        prev = dist;
    }
    counter
}
pub fn calculade_iter(s: &str) -> u32 {
    s.lines()
        .map(|el| el.parse::<u32>().unwrap())
        .fold((0, u32::MAX), |acc: (u32, u32), e| {
            (if e > acc.1 { acc.0 + 1 } else { acc.0 }, e)
        })
        .0
}
pub fn calculate_three(s: &str) -> u32 {
    let data: Vec<u32> = s.lines().map(|el| el.parse::<u32>().unwrap()).collect();
    let len = data.len();
    let mut data_res: Vec<u32> = Vec::new();

    for i in 0..len - 2 {
        data_res.push(data[i] + data[i + 1] + data[i + 2]);
    }
    data_res
        .into_iter()
        .fold((0, u32::MAX), |acc: (u32, u32), e| {
            (if e > acc.1 { acc.0 + 1 } else { acc.0 }, e)
        })
        .0
}
pub fn calculate_three_zip(s: &str) -> u32 {
    let data: Vec<u32> = s.lines().map(|el| el.parse::<u32>().unwrap()).collect();
    let len = data.len();
    let data1 = &data[..len - 2];
    let data2 = &data[1..len - 1];
    let data3 = &data[2..];
    let mut zip = data1.iter().zip(data2.iter()).zip(data3.iter());
    let mut data_res: Vec<u32> = Vec::new();

    while let Some(((a, b), c)) = zip.next() {
        data_res.push(a + b + c);
    }
    data_res
        .into_iter()
        .fold((0, u32::MAX), |acc: (u32, u32), e| {
            (if e > acc.1 { acc.0 + 1 } else { acc.0 }, e)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = "199
200
208
210
200
207
240
269
260
263";
        let result = calculade(inp);
        assert_eq!(result, 7);
    }
    #[test]
    fn it_works_iter() {
        let inp = "199
200
208
210
200
207
240
269
260
263";
        let result = calculade_iter(inp);
        assert_eq!(result, 7);
    }
    #[test]
    fn it_works_three() {
        let inp = "199
200
208
210
200
207
240
269
260
263";
        let result = calculate_three(inp);
        assert_eq!(result, 5);
    }
}
