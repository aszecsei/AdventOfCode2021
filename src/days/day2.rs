use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::internal::Inst;

#[derive(Copy, Clone)]
pub enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

struct Position {
    horiz: i64,
    depth: i64,
    aim: i64,
}

impl Position {
    fn apply(&mut self, instr: Instruction) {
        match instr {
            Instruction::Forward(val) => self.horiz += val,
            Instruction::Down(val) => self.depth += val,
            Instruction::Up(val) => self.depth -= val,
        }
    }

    fn apply_day2(&mut self, instr: Instruction) {
        match instr {
            Instruction::Forward(val) => {
                self.horiz += val;
                self.depth += self.aim * val;
            }
            Instruction::Down(val) => self.aim += val,
            Instruction::Up(val) => self.aim -= val,
        }
    }

    fn get_multiplied(&self) -> i64 {
        self.horiz * self.depth
    }
}

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let spl = x.split(' ').collect_vec();
            let dist = spl[1].trim().parse().unwrap();
            match spl[0] {
                "forward" => Instruction::Forward(dist),
                "down" => Instruction::Down(dist),
                "up" => Instruction::Up(dist),
                _ => unimplemented!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[Instruction]) -> i64 {
    let mut pos = Position {
        horiz: 0,
        depth: 0,
        aim: 0,
    };
    for &instr in input {
        pos.apply(instr);
    }
    pos.get_multiplied()
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[Instruction]) -> i64 {
    let mut pos = Position {
        horiz: 0,
        depth: 0,
        aim: 0,
    };
    for &instr in input {
        pos.apply_day2(instr);
    }
    pos.get_multiplied()
}
