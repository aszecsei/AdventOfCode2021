use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct BingoCell {
    pub value: u32,
    pub marked: bool,
}

#[derive(Clone, Debug)]
pub struct BingoBoard {
    pub cells: [[BingoCell; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
    pub fn value(&self, x: usize, y: usize) -> u32 {
        self.cells[y][x].value
    }
    pub fn marked(&self, x: usize, y: usize) -> bool {
        self.cells[y][x].marked
    }
    pub fn mark(&mut self, value: u32) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.value(x, y) == value {
                    self.cells[y][x].marked = true;
                }
            }
        }
    }

    fn win_vertical(&self) -> bool {
        (0..BOARD_SIZE).any(|x| (0..BOARD_SIZE).all(|y| self.marked(x, y)))
    }

    fn win_horizontal(&self) -> bool {
        (0..BOARD_SIZE).any(|y| (0..BOARD_SIZE).all(|x| self.marked(x, y)))
    }

    fn win(&self) -> bool {
        self.win_vertical() || self.win_horizontal()
    }

    pub fn unmarked_sum(&self) -> u32 {
        (0..BOARD_SIZE)
            .flat_map(|y| (0..BOARD_SIZE).map(move |x| (x, y)))
            .filter(|&(x, y)| !self.marked(x, y))
            .map(|(x, y)| self.value(x, y))
            .sum()
    }
}

#[derive(Clone, Debug)]
pub struct BingoProblem {
    pub numbers: Vec<u32>,
    pub boards: Vec<BingoBoard>,
}

#[aoc_generator(day4)]
pub fn input_generator_day4(input: &str) -> BingoProblem {
    let mut lines = input.lines();

    let numbers = lines.next().unwrap();
    let numbers: Vec<u32> = numbers
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let board_nums: Vec<u32> = lines
        .flat_map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse().unwrap())
        })
        .collect();

    let mut boards = vec![];
    for nums in &board_nums.into_iter().chunks(25) {
        let mut board_data = [[BingoCell::default(); BOARD_SIZE]; BOARD_SIZE];
        for (row_idx, row) in (&nums.into_iter().chunks(5)).into_iter().enumerate() {
            let row_data = row
                .map(|n| BingoCell {
                    value: n,
                    marked: false,
                })
                .collect_vec();
            board_data[row_idx].copy_from_slice(&row_data);
        }
        boards.push(BingoBoard { cells: board_data });
    }

    BingoProblem { numbers, boards }
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(game_data: &BingoProblem) -> u32 {
    let numbers = &game_data.numbers;
    let mut boards = game_data.boards.clone();

    for &n in numbers {
        for b in boards.iter_mut() {
            b.mark(n);
            if b.win() {
                return n * b.unmarked_sum();
            }
        }
    }
    unreachable!();
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(game_data: &BingoProblem) -> u32 {
    let numbers = &game_data.numbers;
    let mut boards = game_data.boards.clone();

    for &n in numbers {
        let board_len = boards.len();
        for b in boards.iter_mut() {
            b.mark(n);
            if board_len == 1 {
                return n * b.unmarked_sum();
            }
        }
        boards.retain(|b| !b.win());
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

    #[test]
    fn test_input_generator() {
        let result = input_generator_day4(DATA);
        assert_eq!(result.numbers[0], 7);
        assert_eq!(result.boards[0].value(0, 0), 22);
        assert_eq!(result.boards[0].value(1, 0), 13);
        assert_eq!(result.boards[0].value(0, 1), 8);
    }

    #[test]
    fn test_board_mark() {
        let mut result = input_generator_day4(DATA);

        result.boards[0].mark(22);
        assert_eq!(result.boards[0].marked(0, 0), true);
    }

    #[test]
    fn test_part1() {
        let problem = input_generator_day4(DATA);
        let result = solve_day4_part1(&problem);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part2() {
        let problem = input_generator_day4(DATA);
        let result = solve_day4_part2(&problem);
        assert_eq!(result, 1924);
    }
}
