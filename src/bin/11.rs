#![feature(test)]

use std::collections::VecDeque;

use advent_of_code::helpers::disjoint_mut_refs_3;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"Monkey \d+:
  Starting items: (\d+(, \d+)*)
  Operation: new = old ([+\-*/]) (old|\d+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"
    )
    .unwrap();
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(u64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: char,
    operand: Operand,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for cap in RE.captures_iter(input) {
        let items = cap[1].split(", ").map(|s| s.parse().unwrap()).collect();
        let op = cap[3].chars().next().unwrap();
        let operand = if let Ok(n) = cap[4].parse() {
            Operand::Number(n)
        } else {
            Operand::Old
        };
        let divisor = cap[5].parse().unwrap();
        let true_monkey = cap[6].parse().unwrap();
        let false_monkey = cap[7].parse().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            operand,
            divisor,
            true_monkey,
            false_monkey,
        });
    }
    monkeys
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0_usize; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (tm, fm) = (monkeys[i].true_monkey, monkeys[i].false_monkey);
            let (monkey, true_monkey, false_monkey) = disjoint_mut_refs_3(&mut monkeys, i, tm, fm);

            inspections[i] += monkey.items.len();

            while let Some(worry) = monkey.items.pop_front() {
                let operand = match monkey.operand {
                    Operand::Old => worry,
                    Operand::Number(n) => n,
                };
                let new_worry = match monkey.op {
                    '+' => worry + operand,
                    '-' => worry - operand,
                    '*' => worry * operand,
                    '/' => worry / operand,
                    _ => unreachable!("invalid operator"),
                };

                let new_worry = new_worry / 3;

                if new_worry % monkey.divisor == 0 {
                    true_monkey.items.push_back(new_worry);
                } else {
                    false_monkey.items.push_back(new_worry);
                }
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0_usize; monkeys.len()];

    let max_mod: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let (tm, fm) = (monkeys[i].true_monkey, monkeys[i].false_monkey);
            let (monkey, true_monkey, false_monkey) = disjoint_mut_refs_3(&mut monkeys, i, tm, fm);

            inspections[i] += monkey.items.len();

            while let Some(worry) = monkey.items.pop_front() {
                let operand = match monkey.operand {
                    Operand::Old => worry,
                    Operand::Number(n) => n,
                };
                let new_worry = match monkey.op {
                    '+' => (worry + operand) % max_mod,
                    '-' => (worry - operand) % max_mod,
                    '*' => (worry * operand) % max_mod,
                    '/' => (worry / operand) % max_mod,
                    _ => unreachable!("invalid operator"),
                };

                if new_worry % monkey.divisor == 0 {
                    true_monkey.items.push_back(new_worry);
                } else {
                    false_monkey.items.push_back(new_worry);
                }
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 11);
        assert_eq!(part_one(&input), Some(54253));
        assert_eq!(part_two(&input), Some(13119526120));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 11);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 11);
        b.iter(|| part_two(input));
    }
}
