use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator_day7(input: &str) -> Vec<u32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &[u32]) -> u32 {
    let mut crabs = input.to_vec();
    crabs.sort_unstable();
    let pos = crabs[crabs.len() / 2];
    crabs
        .iter()
        .map(|&n| u32::max(n, pos) - u32::min(n, pos))
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &[u32]) -> u32 {
    // Average is close but not guaranteed (at most off by one for input ranges given...)
    // but this is the slower, more general solution
    
    let get_cost = |pos: u32| -> u32 {
        input
            .iter()
            .map(|&n| u32::max(n, pos) - u32::min(n, pos))
            .map(|n| n * (n + 1) / 2)
            .sum()
    };

    let pos = (0..*input.iter().max().unwrap()).min_by(|&a, &b| {
        get_cost(a).cmp(&get_cost(b))
    }).unwrap();
    get_cost(pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_day7_part1() {
        let data = input_generator_day7(&DATA);
        let result = solve_day7_part1(&data);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_day7_part2() {
        let data = input_generator_day7(&DATA);
        let result = solve_day7_part2(&data);
        assert_eq!(result, 168);
    }
}
