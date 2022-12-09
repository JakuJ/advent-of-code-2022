#![feature(test)]

use advent_of_code::helpers::disjoint_mut_refs;
use std::collections::hash_set::*;

type Point = (i32, i32);

fn move_head(command: char, head: &mut Point) {
    match command {
        'U' => {
            head.1 += 1;
        }
        'D' => {
            head.1 -= 1;
        }
        'L' => {
            head.0 -= 1;
        }
        'R' => {
            head.0 += 1;
        }
        _ => unreachable!(),
    }
}

fn move_next_knot(head: &Point, tail: &mut Point) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    let adx = dx.abs();
    let ady = dy.abs();

    if adx == 2 || ady == 2 {
        tail.0 += dx.signum();
        tail.1 += dy.signum();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);

    let mut seen = HashSet::new();
    seen.insert(tail);

    for line in input.lines() {
        let command = line.chars().next().unwrap();
        let steps = line.split_at(2).1.parse().unwrap();

        for _ in 0..steps {
            move_head(command, &mut head);
            move_next_knot(&head, &mut tail);

            // update tail
            seen.insert(tail);
        }
    }

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots = [(0_i32, 0_i32); 10];

    let mut seen = HashSet::new();
    seen.insert((0, 0));

    for line in input.lines() {
        let command = line.chars().next().unwrap();
        let steps = line.split_at(2).1.parse().unwrap();

        for _ in 0..steps {
            // move head
            move_head(command, &mut knots[0]);

            for i in 0..9 {
                let (k1, k2) = disjoint_mut_refs(&mut knots[..], i, i + 1);
                move_next_knot(k1, k2);
            }

            // update tail
            seen.insert(knots[9]);
        }
    }

    Some(seen.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 9);
        assert_eq!(part_one(&input), Some(6339));
        assert_eq!(part_two(&input), Some(2541));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 9);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 9);
        b.iter(|| part_two(input));
    }
}
