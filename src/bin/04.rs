#![feature(iter_collect_into)]

use advent_of_code::helpers::parse_with_regex;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let [a, b, c, d] = parse_with_regex::<u32, 4>(&RE, line);
                c >= a && d <= b || a >= c && b <= d
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let [a, b, c, d] = parse_with_regex::<u32, 4>(&RE, line);
                c >= a && d <= b || a >= c && b <= d || c <= a && a <= d || c <= b && d >= b
            })
            .count() as u32,
    )
}

fn main() {
    let _ = RE.is_match(""); // pre-load regex
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
