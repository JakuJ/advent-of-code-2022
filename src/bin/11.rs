#![feature(test)]

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

struct Monkey {
    items: Vec<u64>,
    op: char,
    operand: Option<u64>,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn unchecked_push_vec<T>(vec: &mut Vec<T>, value: T) {
    unsafe {
        vec.as_mut_ptr().add(vec.len()).write(value);
        vec.set_len(vec.len() + 1);
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut capacity = 0;

    for cap in RE.captures_iter(input) {
        let items: Vec<u64> = cap[1].split(", ").map(|s| s.parse().unwrap()).collect();
        capacity += items.len();

        let op = cap[3].chars().next().unwrap();

        let operand = if let Ok(n) = cap[4].parse() {
            Some(n)
        } else {
            None
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

    // ensures that all monkeys have enough capacity to hold all items
    for monkey in &mut monkeys {
        monkey.items.reserve(capacity);
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

            for &worry in monkey.items.iter() {
                let operand = match monkey.operand {
                    None => worry,
                    Some(n) => n,
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
                    true_monkey.items.push(new_worry);
                } else {
                    false_monkey.items.push(new_worry);
                }
            }

            monkey.items.clear();
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    Some(inspections[0] * inspections[1])
}

// 1,273,514 ns/iter (+/- 36,504) -- push_within_capacity
// 1,171,410 ns/iter (+/- 39,868) -- unchecked_push_vec
// 1,066,482 ns/iter (+/- 44,500) -- disjoint_mut_refs_3 without assertions
// 1,054,114 ns/iter (+/- 68,945) -- Option instead of Operand
// 1,042,988 ns/iter (+/- 70,647) -- non-constant items capacity
pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0_usize; monkeys.len()];

    let max_mod: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let (tm, fm) = (monkeys[i].true_monkey, monkeys[i].false_monkey);
            let (monkey, true_monkey, false_monkey) = disjoint_mut_refs_3(&mut monkeys, i, tm, fm);

            inspections[i] += monkey.items.len();

            for &worry in monkey.items.iter() {
                let operand = match monkey.operand {
                    None => worry,
                    Some(n) => n,
                };

                let new_worry = match monkey.op {
                    '+' => worry + operand,
                    '-' => worry - operand,
                    '*' => worry * operand % max_mod,
                    '/' => worry / operand,
                    _ => unreachable!("invalid operator"),
                };

                if new_worry % monkey.divisor == 0 {
                    unchecked_push_vec(&mut true_monkey.items, new_worry);
                } else {
                    unchecked_push_vec(&mut false_monkey.items, new_worry);
                }
            }

            monkey.items.clear();
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
