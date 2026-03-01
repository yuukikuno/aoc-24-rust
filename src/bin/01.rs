use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut first_list, mut second_list): (Vec<u64>, Vec<u64>) = (vec![], vec![]);
    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        first_list.push(first.parse::<u64>().unwrap());
        second_list.push(second.parse::<u64>().unwrap());
    }

    first_list.sort_unstable();
    second_list.sort_unstable();

    let mut result = 0;

    for i in 0..first_list.len() {
        result += first_list[i].abs_diff(second_list[i])
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut first_list: Vec<u64> = vec![];
    // build hashmap with capacity = number of common values (roughly 50 in my input)
    let mut second_uniq: HashMap<u64, u64> = HashMap::with_capacity(50);
    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        first_list.push(first.parse::<u64>().unwrap());
        let second = second.parse::<u64>().unwrap();
        second_uniq
            .entry(second)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let mut result = 0;
    for x in first_list {
        if let Some(count) = second_uniq.get(&x) {
            result += count * x;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
