#![feature(test)]

use advent_of_code::helpers::{disjoint_mut_refs, parse_with_regex};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn parse_crates(input: &[&str]) -> Vec<Vec<char>> {
    let last = *input.last().unwrap();
    let count = last.len() / 4 + 1;

    let mut stacks = vec![Vec::new(); count];

    for line in input.iter().rev().skip(1) {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();

    let stacks: Vec<&str> = lines.by_ref().take_while(|x| !x.is_empty()).collect();
    let mut stacks = parse_crates(&stacks);

    for line in lines {
        let [num, from, to] = parse_with_regex::<usize, 3>(&RE, line);

        let (source, target) = disjoint_mut_refs(&mut stacks, from - 1, to - 1);

        for elem in source.drain(source.len() - num..).rev() {
            target.push(elem);
        }
    }

    Some(stacks.iter().filter_map(|s| s.last()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines();

    let stacks: Vec<&str> = lines.by_ref().take_while(|x| !x.is_empty()).collect();
    let mut stacks = parse_crates(&stacks);

    for line in lines {
        let [num, from, to] = parse_with_regex::<usize, 3>(&RE, line);

        let (source, target) = disjoint_mut_refs(&mut stacks, from - 1, to - 1);

        for elem in source.drain(source.len() - num..) {
            target.push(elem);
        }
    }

    Some(stacks.iter().filter_map(|s| s.last()).collect())
}

fn main() {
    let _ = RE.is_match(""); // pre-load regex
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 5);
        assert_eq!(part_one(&input), Some("CNSZFDVLJ".to_string()));
        assert_eq!(part_two(&input), Some("QNDWLMGNS".to_string()));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 5);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 5);
        b.iter(|| part_two(input));
    }
}
