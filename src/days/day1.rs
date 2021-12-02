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
    input.windows(2).filter(|win| win[0] < win[1]).count() as i64
}

pub fn solve_day1_part1_no_collect(input: &str) -> i64 {
    input
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .fold((0, None), |(acc, a), b| {
            (if a.unwrap_or(b) < b { acc + 1 } else { acc }, Some(b))
        })
        .0
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i64]) -> i64 {
    input.windows(4).filter(|win| win[0] < win[3]).count() as i64
}

pub fn solve_day1_part2_no_collect(input: &str) -> i64 {
    input
        .lines()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .fold((0, None, None, None), |(acc, a, b, c), d| {
            (
                if a.unwrap_or(d) < d { acc + 1 } else { acc },
                b,
                c,
                Some(d),
            )
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    const DATA: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_day1_part1() {
        let data = input_generator_day1(DATA);
        let result = solve_day1_part1(&data);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_day1_part2() {
        let data = input_generator_day1(DATA);
        let result = solve_day1_part2(&data);
        assert_eq!(result, 5);
    }
}
