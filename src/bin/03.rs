advent_of_code::solution!(3);

use core::panic;

use regex::{self, Regex};

const MUL_REGEX: &str = r"mul\((\d+),(\d+)\)";
const MUL_REGEX_WITH_CRABS: &str = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
#[derive(Debug)]
enum Instruction {
    Mul (u64, u64),
    Do,
    Dont
}
#[derive(Debug,PartialEq, Eq)]
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
    for c in re.captures_iter(input) {
        let all_m = c.get(0).unwrap().as_str();
        if all_m.starts_with("do()"){
            out.push(Instruction::Do);
            continue;
        } else if all_m.starts_with("don't()") {
            out.push(Instruction::Dont);
            continue;
        } else { // Mul arm
            let l = c.get(1).unwrap().as_str();
            let r = c.get(2).unwrap().as_str();
            out.push(Instruction::Mul(l.parse().unwrap(),r.parse().unwrap()));
        }
    }
    out
}

fn perform_instruction(inst: Instruction) -> u64 {
    match inst {
        Instruction::Mul(l, r) => l * r,
        Instruction::Do => 0,
        Instruction::Dont => 0,
    }
}

fn process_pt2(data: Vec<Instruction>) -> u64 {
    let mut  state = State::Do; // Initially starts affirmative
    let mut out = 0;
    for d in data {
        let mut tmp: u64 = 0;
        match d {
            Instruction::Mul(l, r) => tmp = l * r,
            Instruction::Do => state = State::Do,
            Instruction::Dont => state = State::Dont
        }
        if state == State::Dont {
            continue
        }
        out += tmp
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_re_mul(input);
    Some(data.into_iter()
    .map(|v| perform_instruction(v))
    .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_re_mul_but_diseased(input);
    Some(process_pt2(data))
    
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
