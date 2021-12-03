use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn get_most_common(input: &[Vec<char>]) -> Vec<char> {
    let mut common_arr = vec!['0'; input[0].len()];

    for ch_pos in 0..input[0].len() {
        let one_count = input.iter().filter(|&s| s[ch_pos] == '1').count();
        if one_count * 2 >= input.len() {
            common_arr[ch_pos] = '1';
        }
    }
    
    common_arr
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Vec<char>]) -> u32 {
    let mut gamma_arr = get_most_common(input);
    
    let gamma_str: String = gamma_arr.iter().collect();
    
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    
    let epsilon_str: String = gamma_arr.iter().map(|&c| if c == '0' { '1' } else { '0' }).collect();
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();
    
    gamma * epsilon
}

pub fn get_common(input: &[Vec<char>], most_common: bool) -> u32 {
    let mut candidates = input.iter().cloned().collect_vec();
    let mut bit_pos = 0;
    while candidates.len() > 1 {
        let common_arr = get_most_common(&candidates);
        candidates.retain(|candidate| {
            if most_common {
                candidate[bit_pos] == common_arr[bit_pos]
            }
            else {
                candidate[bit_pos] != common_arr[bit_pos]
            }
        });
        bit_pos += 1;
    }
    let val_str: String = candidates[0].iter().collect();
    u32::from_str_radix(&val_str, 2).unwrap()
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[Vec<char>]) -> u32 {
    let oxygen = get_common(input, true);
    
    let co2 = get_common(input, false);
    
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    const DATA: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    
    #[test]
    fn test_day3_part1() {
        let data = input_generator_day3(DATA);
        let result = solve_day3_part1(&data);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_day3_part2() {
        let data = input_generator_day3(DATA);
        
        let oxy = get_common(&data, true);
        assert_eq!(oxy, 23);
        
        let co2 = get_common(&data, false);
        assert_eq!(co2, 10);
        
        let result = solve_day3_part2(&data);
        assert_eq!(result, 230);
    }
}
