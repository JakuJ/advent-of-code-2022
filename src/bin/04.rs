#![feature(iter_collect_into)]

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn parse_ranges(line: &str) -> [u32; 4] {
    let caps = RE.captures(line).unwrap();

    [caps.get(1), caps.get(2), caps.get(3), caps.get(4)]
        .map(|x| x.unwrap().as_str().parse().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let [a, b, c, d] = parse_ranges(line);
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
                let [a, b, c, d] = parse_ranges(line);
                c >= a && d <= b || a >= c && b <= d || c <= a && a <= d || c <= b && d >= b
            })
            .count() as u32,
    )
}

fn main() {
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
