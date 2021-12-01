use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// ======================================================
// DAY 1
// ======================================================

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i64]) -> i64 {
    input
        .windows(2)
        .map(|win| if win[1] > win[0] { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i64]) -> i64 {
    input
        .windows(3)
        .map(|win| win.iter().sum::<i64>())
        .collect_vec()
        .windows(2)
        .map(|win| if win[1] > win[0] { 1 } else { 0 })
        .sum()
}

#[test]
fn test_day1_part1() {
    let data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let result = solve_day1_part1(&data);
    assert_eq!(result, 7);
}

#[test]
fn test_day1_part2() {
    let data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let result = solve_day1_part2(&data);
    assert_eq!(result, 5);
}
