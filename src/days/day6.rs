use aoc_runner_derive::{aoc, aoc_generator};

pub type PopulationCounter = [u64; 9];

#[aoc_generator(day6)]
pub fn input_generator_day6(input: &str) -> PopulationCounter {
    let mut pop: PopulationCounter = [0; 9];

    for (idx, age) in pop.iter_mut().enumerate() {
        *age = input
            .split(',')
            .map(|s| s.parse().unwrap())
            .filter(|a: &usize| *a == idx)
            .count() as u64;
    }

    pop
}

fn handle_day(pop: &mut PopulationCounter) {
    let birthing = pop[0];

    for i in 1..9 {
        pop[i - 1] = pop[i];
    }
    pop[6] += birthing; // birthing fish reset to 6
    pop[8] = birthing; // only birthed have counters of 8
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &PopulationCounter) -> u64 {
    let mut pop = *input;

    for _ in 0..80 {
        handle_day(&mut pop);
    }

    pop.iter().sum()
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &PopulationCounter) -> u64 {
    let mut pop = *input;

    for _ in 0..256 {
        handle_day(&mut pop);
    }

    pop.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_input_generator() {
        let result = input_generator_day6(DATA);
        assert_eq!(result, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_day6_part1() {
        let data = input_generator_day6(DATA);
        let result = solve_day6_part1(&data);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_day6_part2() {
        let data = input_generator_day6(DATA);
        let result = solve_day6_part2(&data);
        assert_eq!(result, 26984457539);
    }
}
