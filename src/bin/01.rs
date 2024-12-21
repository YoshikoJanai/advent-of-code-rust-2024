use std::iter::zip;

use itertools::Itertools;



advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lhs: Vec<u64> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();
    for (n, val) in input.split_whitespace()
    .enumerate(){
        if n % 2 == 1{
            lhs.push(val.parse::<u64>().unwrap());
        } else {
            rhs.push(val.parse::<u64>().unwrap());
        }
    }
    lhs.sort();
    rhs.sort();
    
    let sum: u64 = zip(lhs, rhs)
    .map(|(l, r)| -> u64 {
        l.abs_diff(r)
    })
    .sum();
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lhs: Vec<u64> = Vec::new();
    let mut rhs: Vec<u64> = Vec::new();
    for (n, val) in input.split_whitespace()
    .enumerate(){
        if n % 2 == 1{
            lhs.push(val.parse::<u64>().unwrap());
        } else {
            rhs.push(val.parse::<u64>().unwrap());
        }
    }
    let rhs_counts = rhs.into_iter().counts();
    let mut sum: u64 = 0;
    for l in lhs {
        sum = sum + (l * *rhs_counts.get(&l).unwrap_or(&0) as u64)
    }
    Some(sum)
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
