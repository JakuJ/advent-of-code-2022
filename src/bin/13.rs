#![feature(test)]
#![feature(iter_array_chunks)]

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Single(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Single(a), Packet::Single(b)) => a.cmp(b),
            (Packet::Single(a), ls) => Packet::List(vec![Packet::Single(*a)]).cmp(ls),
            (ls, Packet::Single(a)) => ls.cmp(&Packet::List(vec![Packet::Single(*a)])),
            (Packet::List(a), Packet::List(b)) => {
                let mut ai = a.iter();
                let mut bi = b.iter();

                loop {
                    match (ai.next(), bi.next()) {
                        (Some(a), Some(b)) => {
                            let cmp = a.cmp(b);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_packet(input: &str) -> Packet {
    let mut stack = Vec::new();
    let mut current = Vec::new();
    let mut in_number = false;
    let mut number = String::new();

    for c in input.chars() {
        match c {
            '[' => {
                stack.push(current);
                current = Vec::new();
            }
            ']' => {
                if in_number {
                    current.push(Packet::Single(number.parse().unwrap()));
                    number.clear();
                    in_number = false;
                }

                if let Some(mut parent) = stack.pop() {
                    parent.push(Packet::List(current));
                    current = parent;
                }
            }
            ',' => {
                if in_number {
                    current.push(Packet::Single(number.parse().unwrap()));
                    number.clear();
                    in_number = false;
                }
            }
            '0'..='9' => {
                number.push(c);
                in_number = true;
            }
            _ => {}
        }
    }
    current.pop().unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|&x| !x.is_empty())
            .map(parse_packet)
            .array_chunks::<2>()
            .enumerate()
            .map(|(ix, [left, right])| if left < right { ix + 1 } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut trees: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect();

    let divisor = |x| Packet::List(vec![Packet::List(vec![Packet::Single(x)])]);
    let d2 = divisor(2);
    let d6 = divisor(6);

    trees.push(d2.clone());
    trees.push(d6.clone());

    trees.sort_unstable();

    Some(
        trees
            .iter()
            .enumerate()
            .filter(|(_, tree)| tree == &&d2 || tree == &&d6)
            .map(|(ix, _)| ix + 1)
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 13);
        assert_eq!(part_one(&input), Some(5252));
        assert_eq!(part_two(&input), Some(20592));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 13);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 13);
        b.iter(|| part_two(input));
    }
}
