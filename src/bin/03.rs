advent_of_code::solution!(3);

use regex::{self, Regex};

const MUL_REGEX: &str = r"mul\((\d+),(\d+)\)";
const MUL_REGEX_WITH_CRABS: &str = r"mul\((\d)+,(\d+\))|do\(\)|don't\(\)";
enum Instruction {
    Mul (u64, u64),
    Do,
    Dont
}

enum State {
    Do,
    Dont
}

fn parse_re_mul(input: &str) -> Vec<Instruction> {
    let mut out = vec![];
    let re: Regex = Regex::new(MUL_REGEX).expect("{MUL_REGEX} is invalid");
    for (_ , [lhs, rhs]) in re.captures_iter(input).map(|c|c.extract()) {
        out.push(Instruction::Mul(lhs.parse().unwrap(), rhs.parse().unwrap()));
    }
    out
}

fn parse_re_mul_but_diseased(input: &str) -> Vec<Instruction> {
    let mut out = vec![];
    let re = Regex::new(MUL_REGEX_WITH_CRABS).expect("{MUL_REGEX_WITH_CRABS} is invalid");
    
    out
}

fn perform_instruction(inst: Instruction) -> u64 {
    match inst {
        Instruction::Mul(l, r) => l * r,
        Instruction::Do => 0,
        Instruction::Dont => 0,
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_re_mul(input);
    Some(data.into_iter()
    .map(|v| perform_instruction(v))
    .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
