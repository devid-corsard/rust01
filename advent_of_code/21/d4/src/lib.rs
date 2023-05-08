// #[derive(Copy, Clone)]
#[derive(Debug, Clone, Copy)]
struct Num {
    value: u8,
    marked: bool,
}
impl Num {
    fn new() -> Self {
        Self {
            value: 0,
            marked: false,
        }
    }
    fn from(value: u8) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

type BingoData = [[Num; 5]; 5];

type Row<'a> = [&'a Num; 5];
type Col<'a> = [&'a Num; 5];
#[derive(Debug, Clone)]
struct Board<'a> {
    rows: [Row<'a>; 5],
    cols: [Col<'a>; 5],
}

impl<'a> Board<'a> {
    fn from(data: &'a BingoData) -> Self {
        Self {
            rows: [
                [
                    &data[0][0],
                    &data[0][1],
                    &data[0][2],
                    &data[0][3],
                    &data[0][4],
                ],
                [
                    &data[1][0],
                    &data[1][1],
                    &data[1][2],
                    &data[1][3],
                    &data[1][4],
                ],
                [
                    &data[2][0],
                    &data[2][1],
                    &data[2][2],
                    &data[2][3],
                    &data[2][4],
                ],
                [
                    &data[3][0],
                    &data[3][1],
                    &data[3][2],
                    &data[3][3],
                    &data[3][4],
                ],
                [
                    &data[4][0],
                    &data[4][1],
                    &data[4][2],
                    &data[4][3],
                    &data[4][4],
                ],
            ],
            cols: [
                [
                    &data[0][0],
                    &data[1][0],
                    &data[2][0],
                    &data[3][0],
                    &data[4][0],
                ],
                [
                    &data[0][1],
                    &data[1][1],
                    &data[2][1],
                    &data[3][1],
                    &data[4][1],
                ],
                [
                    &data[0][2],
                    &data[1][2],
                    &data[2][2],
                    &data[3][2],
                    &data[4][2],
                ],
                [
                    &data[0][3],
                    &data[1][3],
                    &data[2][3],
                    &data[3][3],
                    &data[4][3],
                ],
                [
                    &data[0][4],
                    &data[1][4],
                    &data[2][4],
                    &data[3][4],
                    &data[4][4],
                ],
            ],
        }
    }
    fn check(&self) -> bool {
        let mut bingo = false;
        self.rows.iter().for_each(|row| {
            if row.iter().all(|el| el.marked == true) {
                bingo = true;
            }
        });
        self.cols.iter().for_each(|col| {
            if col.iter().all(|el| el.marked == true) {
                bingo = true;
            }
        });
        bingo
    }
    fn calculate_winpoints(&self, winnum: u8) -> u32 {
        let mut points = 0;
        self.rows.iter().for_each(|row| {
            row.iter().for_each(|el| {
                if el.marked == false {
                    points += el.value as u32;
                }
            });
        });
        points * winnum as u32
    }
}
fn bingo_data_factory(data: Vec<&str>) -> Vec<BingoData> {
    let mut res: Vec<BingoData> = Vec::new();
    let mut bdd = [[Num::new(); 5]; 5];
    let mut x = 0;
    let mut y = 0;
    for row in data.into_iter() {
        if row.len() == 0 {
            x = 0;
            y = 0;
            continue;
        }
        row.split_whitespace().for_each(|n| {
            bdd[y][x] = Num::from(n.parse::<u8>().unwrap());
            x += 1;
        });
        if y == 4 {
            res.push(bdd.clone());
        }
        x = 0;
        y += 1;
    }
    res
}

pub fn bingo(game: &str) -> u32 {
    let mut game = game.lines();
    let rand_nums = game
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap());
    let mut boards_data: Vec<BingoData> = bingo_data_factory(game.collect());

    let mut winner_board: Option<BingoData> = None;
    for rnum in rand_nums {
        if winner_board.is_some() {
            break;
        }
        for board in boards_data.iter_mut() {
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|num| {
                    if num.value == rnum {
                        num.marked = true;
                    }
                })
            });
            let current_board = Board::from(&board);
            if current_board.check() {
                winner_board = Some(board.clone());
                println!("End of game, with: {rnum}");
                println!("{:#?}", winner_board);
                return current_board.calculate_winpoints(rnum);
            }
        }
    }
    0
}

pub fn bingo_last(game: &str) -> u32 {
    let mut game = game.lines();
    let rand_nums = game
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap());
    let mut boards_data: Vec<BingoData> = bingo_data_factory(game.collect());
    let total_count_of_boards = boards_data.len();
    let mut total_count_win_boards = 0;

    for rnum in rand_nums {
        for board in boards_data.iter_mut() {
            // check if this board already win and skip it
            let current_board = Board::from(&board);
            if current_board.check() {
                continue;
            }
            //mark this num on this board
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|num| {
                    if num.value == rnum {
                        num.marked = true;
                    }
                })
            });
            // chech if after marking new number this board win
            let current_board = Board::from(&board);
            if current_board.check() {
                total_count_win_boards += 1;
            }
            // check if this is a last of poss board
            if total_count_win_boards == total_count_of_boards {
                println!("End of game, with: {rnum}");
                println!("{:#?}", board);
                return current_board.calculate_winpoints(rnum);
            }
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_works() {
        let game = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let result = bingo(game);
        assert_eq!(result, 4512);
    }
    #[test]
    fn p2_works() {
        let game = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let result = bingo_last(game);
        assert_eq!(result, 1924);
    }
}
