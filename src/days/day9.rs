use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::borrow::Borrow;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    x: usize,
    y: usize,
    value: u8,
    basin_id: i32,
}

#[derive(Clone, Debug)]
pub struct Map {
    map: Vec<Vec<u8>>,
    basin_ids: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.map[y][x]
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        Cell {
            x,
            y,
            value: self.map[y][x],
            basin_id: self.basin_ids[y][x],
        }
    }

    pub fn set_basin_id(&mut self, x: usize, y: usize, id: i32) {
        self.basin_ids[y][x] = id;
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_adjacent_cells(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut result = vec![];

        if x > 0 {
            result.push(self.get_cell(x - 1, y));
        }
        if y > 0 {
            result.push(self.get_cell(x, y - 1));
        }
        if x < self.width - 1 {
            result.push(self.get_cell(x + 1, y));
        }
        if y < self.height - 1 {
            result.push(self.get_cell(x, y + 1));
        }

        result
    }

    pub fn get_adjacent_values(&self, x: usize, y: usize) -> Vec<u8> {
        let mut result = vec![];

        if x > 0 {
            result.push(self.get(x - 1, y));
        }
        if y > 0 {
            result.push(self.get(x, y - 1));
        }
        if x < self.width - 1 {
            result.push(self.get(x + 1, y));
        }
        if y < self.height - 1 {
            result.push(self.get(x, y + 1));
        }

        result
    }

    pub fn is_local_min(&self, cell: Cell) -> bool {
        for adj in self.get_adjacent_values(cell.x, cell.y) {
            if adj <= cell.value {
                return false;
            }
        }
        true
    }

    pub fn fill_basin_ids(&mut self) {
        let mut basins = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y);
                if self.is_local_min(cell) {
                    basins.push(cell);
                }
            }
        }

        for (id, cell) in basins.iter().enumerate() {
            let mut to_do = vec![*cell];

            while !to_do.is_empty() {
                let check = to_do.pop().unwrap();
                if check.value != 9 && check.basin_id == -1 {
                    self.set_basin_id(check.x, check.y, id as i32);
                    let mut adjacent_cells = self.get_adjacent_cells(check.x, check.y);
                    to_do.append(&mut adjacent_cells);
                }
            }
        }
    }
}

impl From<Vec<Vec<u8>>> for Map {
    fn from(v: Vec<Vec<u8>>) -> Self {
        let height = v.len();
        let width = v[0].len();
        Map {
            map: v,
            basin_ids: vec![vec![-1; width]; height],
            height,
            width,
        }
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = Cell;
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator {
            map: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct MapIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.map.is_in_bounds(self.x, self.y) {
            return None;
        }
        let cell = self.map.get_cell(self.x, self.y);
        self.x += 1;
        if self.x >= self.map.width {
            self.x = 0;
            self.y += 1;
        }
        Some(cell)
    }
}

#[aoc_generator(day9)]
pub fn input_generator_day9(input: &str) -> Map {
    input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
        .into()
}

#[aoc(day9, part1)]
pub fn solve_day9_part1(input: &Map) -> usize {
    input
        .into_iter()
        .filter(|&cell| input.is_local_min(cell))
        .map(|cell| cell.value as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(input: &Map) -> usize {
    let mut input = input.clone();
    input.fill_basin_ids();

    let max_basin_id = input
        .borrow()
        .into_iter()
        .map(|cell| cell.basin_id)
        .max()
        .unwrap();
    let mut counts = vec![0; max_basin_id as usize + 1];
    for cell in &input {
        if cell.basin_id > 0 {
            counts[cell.basin_id as usize] += 1;
        }
    }
    let mut counts = counts.into_iter().enumerate().collect_vec();
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    counts[0].1 * counts[1].1 * counts[2].1
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_day9_part1() {
        let data = input_generator_day9(&DATA);
        let result = solve_day9_part1(&data);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day9_part2() {
        let data = input_generator_day9(&DATA);
        let result = solve_day9_part2(&data);
        assert_eq!(result, 1134);
    }
}
