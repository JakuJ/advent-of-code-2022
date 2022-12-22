#![feature(test)]

use std::collections::HashMap;
use z3::ast::Ast;

#[derive(Clone)]
enum Value {
    Const(i64),
    App(String, char, String),
}

fn parse_line(line: &str) -> (String, Value) {
    let monkey = line[..4].to_string();
    if line.len() == 17 {
        let left = line[6..=9].to_string();
        let op = line.chars().nth(11).unwrap();
        let right = line[13..=16].to_string();
        (monkey, Value::App(left, op, right))
    } else {
        let val = line[6..].parse().unwrap();
        (monkey, Value::Const(val))
    }
}

fn solve(monkey: String, monkeys: &mut HashMap<String, Value>) -> i64 {
    let rhs = monkeys.get(&monkey).unwrap().clone();
    match rhs {
        Value::Const(v) => v,
        Value::App(left, op, right) => {
            let left = solve(left, monkeys);
            let right = solve(right, monkeys);

            let val = match op {
                '+' => left + right,
                '*' => left * right,
                '-' => left - right,
                '/' => left / right,
                _ => unreachable!(),
            };

            monkeys.insert(monkey, Value::Const(val));
            val
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = input.lines().map(parse_line).collect();
    Some(solve("root".to_string(), &mut monkeys))
}

pub fn part_two(input: &str) -> Option<i64> {
    let monkeys = input.lines().map(parse_line).collect::<Vec<_>>();

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    // declare variables for each monkey
    let monkey_vars: HashMap<String, z3::ast::Real> = monkeys
        .iter()
        .map(|(name, _)| (name.clone(), z3::ast::Real::new_const(&ctx, name.clone())))
        .collect();

    // make assertions for all monkeys
    for (name, value) in monkeys.iter() {
        if name == "humn" {
            // ignore human
        } else if name == "root" {
            if let Value::App(left, _, right) = value {
                let root_left = monkey_vars.get(left).unwrap();
                let root_right = monkey_vars.get(right).unwrap();
                solver.assert(&root_left._eq(root_right));
            }
        } else {
            let monkey = monkey_vars.get(name).unwrap();
            match value {
                Value::Const(v) => {
                    solver.assert(
                        &monkey._eq(&z3::ast::Real::from_int(&z3::ast::Int::from_i64(&ctx, *v))),
                    );
                }
                Value::App(left, op, right) => {
                    let left = monkey_vars.get(left).unwrap();
                    let right = monkey_vars.get(right).unwrap();

                    let val = match op {
                        '+' => left + right,
                        '*' => left * right,
                        '-' => left - right,
                        '/' => left / right,
                        _ => unreachable!(),
                    };

                    solver.assert(&monkey._eq(&val));
                }
            }
        }
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let (n, d) = solver
                .get_model()
                .unwrap()
                .eval(&monkey_vars["humn"], true)
                .unwrap()
                .as_real()
                .unwrap();
            Some(n / d)
        }
        _ => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 21);
        assert_eq!(part_one(&input), Some(72664227897438));
        assert_eq!(part_two(&input), Some(3916491093817));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 21);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 21);
        b.iter(|| part_two(input));
    }
}
