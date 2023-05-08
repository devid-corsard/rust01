pub fn go_aim(course: &str) -> u32 {
    let course = course.lines().map(|el| {
        let mut command = el.split_whitespace();
        let direction = command.next().unwrap();
        let distance = command.next().unwrap().parse::<u32>().unwrap();
        (direction, distance)
    });
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    course.for_each(|command| match command {
        ("down", val) => aim += val,
        ("up", val) => aim -= val,
        ("forward", val) => {
            h_pos += val;
            depth += aim * val;
        }
        _ => (),
    });
    h_pos * depth
}
pub fn go(course: &str) -> u32 {
    let course = course.lines().map(|el| {
        let mut command = el.split_whitespace();
        let direction = command.next().unwrap();
        let distance = command.next().unwrap().parse::<u32>().unwrap();
        (direction, distance)
    });
    let mut h_pos = 0;
    let mut depth = 0;
    course.for_each(|command| match command {
        ("down", val) => depth += val,
        ("up", val) => depth -= val,
        ("forward", val) => h_pos += val,
        _ => (),
    });
    h_pos * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_works() {
        let course = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let result = go(course);
        assert_eq!(result, 150);
    }
    #[test]
    fn p2_works() {
        let course = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let result = go_aim(course);
        assert_eq!(result, 900);
    }
}
