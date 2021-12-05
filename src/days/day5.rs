use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
pub struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[aoc_generator(day5)]
pub fn input_generator_day5(input: &str) -> Vec<(Point2D, Point2D)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) \-> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    input
        .lines()
        .map(|x| {
            let caps = RE.captures(x.trim()).unwrap();
            (
                Point2D::new(caps["x1"].parse().unwrap(), caps["y1"].parse().unwrap()),
                Point2D::new(caps["x2"].parse().unwrap(), caps["y2"].parse().unwrap()),
            )
        })
        .collect()
}

pub fn get_intersections(input: &[(Point2D, Point2D)], allow_diagonals: bool) -> usize {
    let mut points_cnt = HashMap::new();

    for (pt1, pt2) in input {
        let length_x = pt2.x - pt1.x;
        let length_y = pt2.y - pt1.y;

        if !allow_diagonals && length_x != 0 && length_y != 0 {
            // If we don't allow diagonals, either length_x or length_y should be zero
            continue;
        }

        let length = i32::max(i32::abs(length_x), i32::abs(length_y));
        let delta_x = i32::max(i32::min(length_x, 1), -1);
        let delta_y = i32::max(i32::min(length_y, 1), -1);

        for pt in
            (0..=length).map(|dist| Point2D::new(pt1.x + dist * delta_x, pt1.y + dist * delta_y))
        {
            let value = points_cnt.entry(pt).or_insert_with(|| 0);
            *value += 1;
        }
    }

    let common_points = points_cnt
        .iter()
        .filter(|(_point, cnt)| **cnt > 1)
        .map(|(point, _cnt)| *point)
        .collect_vec();

    common_points.len()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &[(Point2D, Point2D)]) -> usize {
    get_intersections(input, false)
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &[(Point2D, Point2D)]) -> usize {
    get_intersections(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_input_generator() {
        let result = input_generator_day5(DATA);
        assert_eq!(result[0], (Point2D::new(0, 9), Point2D::new(5, 9)));
        assert_eq!(result[1], (Point2D::new(8, 0), Point2D::new(0, 8)));
    }

    #[test]
    fn test_day5_part1() {
        let data = input_generator_day5(DATA);
        let result = solve_day5_part1(&data);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_day5_part2() {
        let data = input_generator_day5(DATA);
        let result = solve_day5_part2(&data);
        assert_eq!(result, 12);
    }
}
