use aoc_runner_derive::{aoc, aoc_generator};
use enumflags2::{bitflags, BitFlags};
use itertools::Itertools;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SignalLine {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = 1 << 5,
    G = 1 << 6,
}

impl SignalLine {
    pub fn from_char(ch: char) -> SignalLine {
        match ch {
            'a' => SignalLine::A,
            'b' => SignalLine::B,
            'c' => SignalLine::C,
            'd' => SignalLine::D,
            'e' => SignalLine::E,
            'f' => SignalLine::F,
            'g' => SignalLine::G,
            _ => panic!("Unexpected character '{}'", ch),
        }
    }

    pub fn from_string(input: &str) -> BitFlags<SignalLine> {
        let mut result = BitFlags::default();
        for ch in input.chars() {
            result |= Self::from_char(ch);
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub patterns: [BitFlags<SignalLine>; 10],
    pub output: [BitFlags<SignalLine>; 4],
}

pub fn is_unique(signal: &BitFlags<SignalLine>) -> bool {
    let count = signal.iter().count();
    //              1 | 7 | 4 | 8
    matches!(count, 2 | 3 | 4 | 7)
}

#[aoc_generator(day8)]
pub fn input_generator_day8(input: &str) -> Vec<Entry> {
    input
        .split('\n')
        .map(|s| {
            let parts = s.split('|').collect_vec();
            let patterns_vec = parts[0]
                .trim()
                .split(' ')
                .map(|s| SignalLine::from_string(s))
                .collect_vec();
            let output_vec = parts[1]
                .trim()
                .split(' ')
                .map(|s| SignalLine::from_string(s))
                .collect_vec();

            let mut patterns = [BitFlags::default(); 10];
            let mut output = [BitFlags::default(); 4];
            patterns.copy_from_slice(&patterns_vec);
            output.copy_from_slice(&output_vec);

            Entry { patterns, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|e| e.output.iter().filter(|&b| is_unique(b)).count())
        .sum()
}

#[allow(clippy::many_single_char_names)]
pub fn solve_map(entry: &Entry) -> [BitFlags<SignalLine>; 10] {
    let mut result = [BitFlags::default(); 10];

    let mut unsolved_patterns = entry.patterns.to_vec();

    // start with the (easy) unique counts
    for p in unsolved_patterns.iter() {
        match p.iter().count() {
            2 => result[1] = *p,
            3 => result[7] = *p,
            4 => result[4] = *p,
            7 => result[8] = *p,
            _ => (),
        };
    }
    unsolved_patterns.retain(|p| !result.contains(p));

    // wire a shows up in 8 patterns (7 and 8 unique)
    // wire b shows up in 6 patterns (4 and 8 unique)         -- can determine
    // wire c shows up in 8 patterns (all unique)
    // wire d shows up in 7 patterns (4 and 8 unique)
    // wire e shows up in 4 patterns (8 unique)               -- can determine
    // wire f shows up in 9 patterns (all unique)             -- can determine
    // wire g shows up in 7 patterns (8 unique)

    // WIRE     TOTAL / REMAINING
    // a        8 / 6
    // b        6 / 4
    // c        8 / 4
    // d        7 / 5
    // e        4 / 3
    // f        9 / 5
    // g        7 / 6

    let wires = [
        SignalLine::A,
        SignalLine::B,
        SignalLine::C,
        SignalLine::D,
        SignalLine::E,
        SignalLine::F,
        SignalLine::G,
    ];
    let counts = wires
        .iter()
        .map(|w| {
            (
                *w,
                entry.patterns.iter().filter(|p| p.contains(*w)).count(),
                unsolved_patterns.iter().filter(|p| p.contains(*w)).count(),
            )
        })
        .collect_vec();

    let a = counts
        .iter()
        .find(|(_wire, t, r)| *t == 8 && *r == 6)
        .unwrap()
        .0;
    let b = counts
        .iter()
        .find(|(_wire, t, r)| *t == 6 && *r == 4)
        .unwrap()
        .0;
    let c = counts
        .iter()
        .find(|(_wire, t, r)| *t == 8 && *r == 4)
        .unwrap()
        .0;
    let d = counts
        .iter()
        .find(|(_wire, t, r)| *t == 7 && *r == 5)
        .unwrap()
        .0;
    let e = counts
        .iter()
        .find(|(_wire, t, r)| *t == 4 && *r == 3)
        .unwrap()
        .0;
    let f = counts
        .iter()
        .find(|(_wire, t, r)| *t == 9 && *r == 5)
        .unwrap()
        .0;
    let g = counts
        .iter()
        .find(|(_wire, t, r)| *t == 7 && *r == 6)
        .unwrap()
        .0;

    // NEED TO FIND:
    // 0, 2, 3, 5, 6, 9

    result[0] = a | b | c | e | f | g;
    result[2] = a | c | d | e | g;
    result[3] = a | c | d | f | g;
    result[5] = a | b | d | f | g;
    result[6] = a | b | d | e | f | g;
    result[9] = a | b | c | d | f | g;

    result
}

pub fn get_digit(signal: BitFlags<SignalLine>, map: &[BitFlags<SignalLine>]) -> usize {
    for (i, s) in map.iter().enumerate() {
        if *s == signal {
            return i;
        }
    }
    panic!("Could not find digit!");
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|e| {
            let map = solve_map(e);
            get_digit(e.output[0], &map) * 1000
                + get_digit(e.output[1], &map) * 100
                + get_digit(e.output[2], &map) * 10
                + get_digit(e.output[3], &map)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const SHORT_DATA: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_is_unique() {
        let numbers = [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];
        let should_be_unique = [
            false, true, false, false, true, false, false, true, true, false,
        ];

        for (&num, &unique) in numbers.iter().zip(should_be_unique.iter()) {
            let bf = SignalLine::from_string(num);
            assert_eq!(is_unique(&bf), unique);
        }
    }

    #[test]
    fn test_day8_part1() {
        let data = input_generator_day8(&DATA);
        let result = solve_day8_part1(&data);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_solve_map() {
        let data = input_generator_day8(&SHORT_DATA);
        let map = solve_map(&data[0]);
        let correct = [
            SignalLine::from_string("cagedb"),
            SignalLine::from_string("ab"),
            SignalLine::from_string("gcdfa"),
            SignalLine::from_string("fbcad"),
            SignalLine::from_string("eafb"),
            SignalLine::from_string("cdfbe"),
            SignalLine::from_string("cdfgeb"),
            SignalLine::from_string("dab"),
            SignalLine::from_string("acedgfb"),
            SignalLine::from_string("cefabd"),
        ];
        assert_eq!(map, correct);
    }

    #[test]
    fn test_day8_part2() {
        let data = input_generator_day8(&DATA);
        let result = solve_day8_part2(&data);
        assert_eq!(result, 61229);
    }
}
