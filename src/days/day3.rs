use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> (Vec<u16>, u8) {
    (
        input
            .lines()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect(),
        input.lines().max_by_key(|s| s.len()).unwrap().len() as u8,
    )
}

pub fn is_bit_set(num: u16, pos: u8) -> bool {
    num & (1 << pos) != 0
}

pub fn is_same_bit(a: u16, b: u16, pos: u8) -> bool {
    (a & (1 << pos)) ^ (b & (1 << pos)) == 0
}

pub fn get_most_common(input: &[u16], len: u8) -> (u16, u16) {
    let mut common = 0;
    let mut uncommon = 0;

    assert!(len < 16);
    for bit_pos in 0..len {
        let one_count = input.iter().filter(|&&s| is_bit_set(s, bit_pos)).count();
        let zero_count = input.len() - one_count;
        if one_count >= zero_count {
            common |= 1 << bit_pos;
        } else {
            uncommon |= 1 << bit_pos;
        }
    }

    (common, uncommon)
}

#[aoc(day3, part1)]
pub fn solve_day3_part1((input, len): &(Vec<u16>, u8)) -> u32 {
    let (gamma, epsilon) = get_most_common(input, *len);
    gamma as u32 * epsilon as u32
}

pub fn get_element(input: &[u16], most_common: bool, len: u8) -> u16 {
    let mut candidates = input.to_vec();
    let mut bit_pos = len;
    while candidates.len() > 1 {
        bit_pos -= 1;
        let (common, uncommon) = get_most_common(&candidates, len);
        let compare = if most_common { common } else { uncommon };
        candidates.retain(|&candidate| is_same_bit(candidate, compare, bit_pos));
    }
    candidates[0]
}

#[aoc(day3, part2)]
pub fn solve_day3_part2((input, len): &(Vec<u16>, u8)) -> u32 {
    let oxygen = get_element(input, true, *len);
    let co2 = get_element(input, false, *len);

    oxygen as u32 * co2 as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_is_bit_set() {
        const VAL: u16 = 0b10100;
        assert_eq!(is_bit_set(VAL, 0), false);
        assert_eq!(is_bit_set(VAL, 1), false);
        assert_eq!(is_bit_set(VAL, 2), true);
        assert_eq!(is_bit_set(VAL, 3), false);
        assert_eq!(is_bit_set(VAL, 4), true);
    }

    #[test]
    fn test_is_same_bit() {
        const V1: u16 = 0b10100;
        const V2: u16 = 0b00110;
        assert_eq!(is_same_bit(V1, V2, 0), true);
        assert_eq!(is_same_bit(V1, V2, 1), false);
        assert_eq!(is_same_bit(V1, V2, 2), true);
        assert_eq!(is_same_bit(V1, V2, 3), true);
        assert_eq!(is_same_bit(V1, V2, 4), false);
    }

    #[test]
    fn test_get_most_common() {
        const VAL: [u16; 2] = [0b10110, 0b10111];
        let (common, uncommon) = get_most_common(&VAL, 5);

        assert_eq!(common, 0b10111);
        assert_eq!(uncommon, 0b01000);
    }

    #[test]
    fn test_day3_part1() {
        let data = input_generator_day3(DATA);
        let result = solve_day3_part1(&data);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_day3_part2() {
        let data = input_generator_day3(DATA);

        let oxy = get_element(&data.0, true, data.1);
        assert_eq!(oxy, 23);

        let co2 = get_element(&data.0, false, data.1);
        assert_eq!(co2, 10);

        let result = solve_day3_part2(&data);
        assert_eq!(result, 230);
    }
}
