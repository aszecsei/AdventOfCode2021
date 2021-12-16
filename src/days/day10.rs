use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TokenType {
    Paren,
    Square,
    Curly,
    Angle,
}

impl TokenType {
    pub fn illegal_value(self) -> u32 {
        match self {
            TokenType::Paren => 3,
            TokenType::Square => 57,
            TokenType::Curly => 1197,
            TokenType::Angle => 25137,
        }
    }

    pub fn completion_value(self) -> u32 {
        match self {
            TokenType::Paren => 1,
            TokenType::Square => 2,
            TokenType::Curly => 3,
            TokenType::Angle => 4,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Token {
    ty: TokenType,
    is_open: bool,
}

impl Token {
    pub fn from_char(ch: char) -> Token {
        match ch {
            '(' => Token {
                ty: TokenType::Paren,
                is_open: true,
            },
            ')' => Token {
                ty: TokenType::Paren,
                is_open: false,
            },
            '[' => Token {
                ty: TokenType::Square,
                is_open: true,
            },
            ']' => Token {
                ty: TokenType::Square,
                is_open: false,
            },
            '{' => Token {
                ty: TokenType::Curly,
                is_open: true,
            },
            '}' => Token {
                ty: TokenType::Curly,
                is_open: false,
            },
            '<' => Token {
                ty: TokenType::Angle,
                is_open: true,
            },
            '>' => Token {
                ty: TokenType::Angle,
                is_open: false,
            },
            _ => panic!("Unexpected character!"),
        }
    }

    pub fn is_match(&self, other: Token) -> bool {
        self.ty == other.ty && self.is_open != other.is_open
    }
}

pub struct Line {
    tokens: Vec<Token>,
}

impl Line {
    pub fn illegal(&self) -> u32 {
        let mut stack = vec![];
        for tok in &self.tokens {
            if tok.is_open {
                stack.push(tok);
            } else {
                let other = stack.pop();
                match other {
                    Some(other_token) if tok.is_match(*other_token) => (),
                    _ => return tok.ty.illegal_value(),
                }
            }
        }
        0
    }

    pub fn is_illegal(&self) -> bool {
        let mut stack = vec![];
        for tok in &self.tokens {
            if tok.is_open {
                stack.push(tok);
            } else {
                let other = stack.pop();
                match other {
                    Some(other_token) if tok.is_match(*other_token) => (),
                    _ => return true,
                }
            }
        }
        false
    }

    pub fn autocomplete(&self) -> u64 {
        let mut stack = vec![];
        for tok in &self.tokens {
            if tok.is_open {
                stack.push(tok);
            } else {
                stack.pop();
            }
        }

        let mut score = 0;
        while !stack.is_empty() {
            score *= 5;
            let tok = stack.pop().unwrap();
            score += tok.ty.completion_value() as u64;
        }
        score
    }
}

impl From<Vec<Token>> for Line {
    fn from(tokens: Vec<Token>) -> Self {
        Line { tokens }
    }
}

#[aoc_generator(day10)]
pub fn input_generator_day10(input: &str) -> Vec<Line> {
    input
        .split('\n')
        .map(|s| s.chars().map(Token::from_char).collect_vec().into())
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &[Line]) -> u32 {
    input.iter().map(|l| l.illegal()).sum()
}

#[aoc(day10, part2)]
pub fn solve_day10_part2(input: &[Line]) -> u64 {
    let mut scores = input
        .iter()
        .filter(|l| !l.is_illegal())
        .map(|l| l.autocomplete())
        .collect_vec();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_day10_part1() {
        let data = input_generator_day10(&DATA);
        let result = solve_day10_part1(&data);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_day10_part2() {
        let data = input_generator_day10(&DATA);
        let result = solve_day10_part2(&data);
        assert_eq!(result, 288957);
    }
}
