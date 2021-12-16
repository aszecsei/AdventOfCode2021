use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Grid {
    energy: [[u8; 10]; 10],
    flashes: usize,
}

impl Grid {
    fn is_in_bounds(x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < 10 && y < 10
    }

    const ADJACENCY: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn simulate(&mut self) {
        for y in 0..10 {
            for x in 0..10 {
                self.energy[y][x] += 1;
            }
        }

        let mut flashing = vec![];
        for y in 0..10 {
            for x in 0..10 {
                if self.energy[y][x] > 9 {
                    flashing.push((x, y));
                }
            }
        }

        while !flashing.is_empty() {
            let handle = flashing.pop().unwrap();
            self.flashes += 1;
            for adj in &Self::ADJACENCY {
                let ax = handle.0 as i32 + adj.0;
                let ay = handle.1 as i32 + adj.1;
                if Self::is_in_bounds(ax, ay) {
                    let ax = ax as usize;
                    let ay = ay as usize;

                    if self.energy[ay][ax] > 9 {
                        continue;
                    }

                    self.energy[ay][ax] += 1;
                    if self.energy[ay][ax] > 9 {
                        flashing.push((ax, ay));
                    }
                }
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                if self.energy[y][x] > 9 {
                    self.energy[y][x] = 0;
                }
            }
        }
    }

    pub fn all_flashed(&self) -> bool {
        self.energy.iter().all(|l| l.iter().all(|&e| e == 0))
    }

    pub fn flashes(&self) -> usize {
        self.flashes
    }
}

#[aoc_generator(day11)]
pub fn input_generator_day11(input: &str) -> Grid {
    let mut grid = [[0; 10]; 10];
    input.split('\n').enumerate().for_each(|(i, s)| {
        let row_vec = s
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect_vec();
        grid[i].copy_from_slice(&row_vec);
    });
    Grid {
        energy: grid,
        flashes: 0,
    }
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(input: &Grid) -> usize {
    let mut input = input.clone();
    for _ in 0..100 {
        input.simulate();
    }
    input.flashes()
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(input: &Grid) -> usize {
    let mut input = input.clone();
    let mut counter = 0;
    loop {
        input.simulate();
        counter += 1;

        if input.all_flashed() {
            break;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_day11_part1() {
        let data = input_generator_day11(&DATA);
        let result = solve_day11_part1(&data);
        assert_eq!(result, 1656);
    }

    #[test]
    fn test_day11_part2() {
        let data = input_generator_day11(&DATA);
        let result = solve_day11_part2(&data);
        assert_eq!(result, 195);
    }
}
