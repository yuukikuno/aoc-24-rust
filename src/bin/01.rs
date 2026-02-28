advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut first_list, mut second_list): (Vec<u64>, Vec<u64>) = (vec![], vec![]);
    // build lists
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let first = split.next()?.parse::<u64>().unwrap();
        let second = split.next()?.parse::<u64>().unwrap();
        first_list.push(first);
        second_list.push(second);
    }

    first_list.sort();
    second_list.sort();

    let zipped = first_list.into_iter().zip(second_list);

    let mut result = 0;

    for (first, second) in zipped {
        result += first.abs_diff(second);
    }
    Some(result)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
