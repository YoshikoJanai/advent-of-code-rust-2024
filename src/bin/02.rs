use advent_of_code::utils::logical_xor;
advent_of_code::solution!(2);

fn validate_report(input: &Vec<u64>) -> u8{
    let descending: bool = input.is_sorted_by(|a,b| a < b);
    let ascending: bool = input.is_sorted_by(|a ,b| a > b);
    let pre_check: bool = logical_xor(descending, ascending);
    if ! pre_check{
       return 0
    }

    for w in input.windows(2){
        let tmp = w.to_vec();
        let diff = tmp.into_iter()
        .reduce(|acc: u64, e: u64| -> u64 {
            acc.abs_diff(e)
        })
        .unwrap();
        
        if diff > 3 {
            return 0
        }
    }
    1
}

fn generate_permutations(input: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut out: Vec<Vec<u64>> = Vec::new();
    
    let count = input.len();
    for i in 0..count {
        let mut tmp = input.clone();
        tmp.remove(i);
        out.push(tmp);
    }
    out
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<Vec<u64>> = Vec::new();

    for r in input.split("\n") {
        let mut tmp: Vec<u64> = Vec::new();
        for n in r.split_whitespace() {
            tmp.push(n.parse::<u64>().unwrap());
        }
        data.push(tmp);
    }
    Some(data.into_iter()
    .fold(0, |acc: u64, e| {
        acc + (validate_report(&e) as u64)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data: Vec<Vec<u64>> = Vec::new();

    for r in input.split("\n") {
        let mut tmp: Vec<u64> = Vec::new();
        for n in r.split_whitespace() {
            tmp.push(n.parse::<u64>().unwrap());
        }
        data.push(tmp);
    }
    Some(data.into_iter()
        .map(|v| {
            let tmp = generate_permutations(&v);
            tmp.into_iter()
            .fold(0, |acc: u64, e| {
                acc | (validate_report(&e) as u64)
            })
        })
        .sum())
        
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
