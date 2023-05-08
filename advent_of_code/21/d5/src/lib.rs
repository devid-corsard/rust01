use std::{collections::HashMap, fmt};

#[derive(Clone, Copy, Hash, Eq)]
struct Point {
    x: u16,
    y: u16,
}
impl Point {
    fn from(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
struct Line {
    start: Point,
    end: Point,
}
impl Line {
    fn from(start: Point, end: Point) -> Self {
        Self { start, end }
    }
    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
    fn get_all_points(&self) -> Vec<Point> {
        let mut line: Vec<Point> = Vec::new();
        let [x1, x2, y1, y2] = [self.start.x, self.end.x, self.start.y, self.end.y];
        match [x2 as i16 - x1 as i16, y2 as i16 - y1 as i16] {
            [dx, 0] if dx >= 0 => {
                for x in x1..=x2 {
                    line.push(Point::from(x, y1));
                }
            }
            [dx, 0] if dx < 0 => {
                for x in x2..=x1 {
                    line.push(Point::from(x, y1));
                }
            }
            [0, dy] if dy > 0 => {
                for y in y1..=y2 {
                    line.push(Point::from(x1, y));
                }
            }
            [0, dy] if dy < 0 => {
                for y in y2..=y1 {
                    line.push(Point::from(x1, y));
                }
            }
            [dx, dy] if dx > 0 && dy > 0 => {
                for (x, y) in (x1..=x2).zip(y1..=y2) {
                    line.push(Point::from(x, y));
                }
            }
            [dx, dy] if dx > 0 && dy < 0 => {
                for (x, y) in (x1..=x2).zip((y2..=y1).rev()) {
                    line.push(Point::from(x, y));
                }
            }
            [dx, dy] if dx < 0 && dy > 0 => {
                for (x, y) in (x2..=x1).rev().zip(y1..=y2) {
                    line.push(Point::from(x, y));
                }
            }
            [dx, dy] if dx < 0 && dy < 0 => {
                for (x, y) in (x2..=x1).zip(y2..=y1) {
                    line.push(Point::from(x, y));
                }
            }
            _ => println!("ooops"),
        }
        line
    }
}
pub fn danger_points(data: &str) -> u32 {
    //parse
    let parse_line = |l: &str| {
        let mut line_data = l.split(" -> ");
        let (p1, p2) = (line_data.next().unwrap(), line_data.next().unwrap());
        let mut p1_data = p1.split(',');
        let (x1, y1): (u16, u16) = (
            p1_data.next().unwrap().parse::<u16>().unwrap(),
            p1_data.next().unwrap().parse::<u16>().unwrap(),
        );
        let mut p2_data = p2.split(',');
        let (x2, y2): (u16, u16) = (
            p2_data.next().unwrap().parse::<u16>().unwrap(),
            p2_data.next().unwrap().parse::<u16>().unwrap(),
        );
        let start = Point::from(x1, y1);
        let end = Point::from(x2, y2);
        Line::from(start, end)
    };
    //filter only 90deg lines
    let lines: Vec<Line> = data
        .lines()
        .map(parse_line)
        .filter(|line| line.is_straight())
        .collect();
    //calculate all between points
    let mut diagram: HashMap<Point, bool> = HashMap::new();

    for line in lines.iter() {
        for point in line.get_all_points().into_iter() {
            diagram
                .entry(point)
                //calculate intersections of points
                .and_modify(|v| *v = true)
                .or_insert(false);
        }
    }
    //calculate sum of 3 + intersections
    diagram.values().filter(|&&val| val).count() as u32

    // 0
}
pub fn danger_points_all(data: &str) -> u32 {
    //parse
    let parse_line = |l: &str| {
        let mut line_data = l.split(" -> ");
        let (p1, p2) = (line_data.next().unwrap(), line_data.next().unwrap());
        let mut p1_data = p1.split(',');
        let (x1, y1): (u16, u16) = (
            p1_data.next().unwrap().parse::<u16>().unwrap(),
            p1_data.next().unwrap().parse::<u16>().unwrap(),
        );
        let mut p2_data = p2.split(',');
        let (x2, y2): (u16, u16) = (
            p2_data.next().unwrap().parse::<u16>().unwrap(),
            p2_data.next().unwrap().parse::<u16>().unwrap(),
        );
        let start = Point::from(x1, y1);
        let end = Point::from(x2, y2);
        Line::from(start, end)
    };
    let lines: Vec<Line> = data.lines().map(parse_line).collect();
    //calculate all between points
    let mut diagram: HashMap<Point, bool> = HashMap::new();

    for line in lines.iter() {
        for point in line.get_all_points().into_iter() {
            diagram
                .entry(point)
                //calculate intersections of points
                .and_modify(|v| *v = true)
                .or_insert(false);
        }
    }
    //calculate sum of 3 + intersections
    // println!("{:?}", diagram);
    diagram.values().filter(|&&val| val).count() as u32
    // 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_works() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = danger_points(input);
        assert_eq!(result, 5);
    }
    #[test]
    fn p2_works() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = danger_points_all(input);
        assert_eq!(result, 12);
    }
}
