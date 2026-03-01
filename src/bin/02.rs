use itertools::Itertools;

advent_of_code::solution!(2);

enum Direction {
    Ascending,
    Descending,
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    'report: for report in input.lines() {
        let mut iterator = report
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .tuple_windows();
        let (first, second) = iterator.next().unwrap();
        if first.abs_diff(second) > 3 {
            continue 'report;
        }
        let direction = match first.cmp(&second) {
            std::cmp::Ordering::Less => Direction::Ascending,
            std::cmp::Ordering::Equal => continue 'report,
            std::cmp::Ordering::Greater => Direction::Descending,
        };
        for (a, b) in iterator {
            if a.abs_diff(b) > 3 {
                continue 'report;
            }
            match (&direction, a.cmp(&b)) {
                (Direction::Ascending, std::cmp::Ordering::Less) => (),
                (Direction::Descending, std::cmp::Ordering::Greater) => (),
                (_, _) => continue 'report,
            }
        }
        result += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
