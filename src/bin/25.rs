#![feature(test)]

use z3::ast::Ast;

fn snafu_to_decimal(snafu: String) -> i64 {
    let mut decimal = 0;
    let mut power = 1;
    for c in snafu.chars().rev() {
        decimal += power
            * match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
        power *= 5;
    }
    decimal
}

fn decimal_to_snafu(decimal: i64) -> String {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let max_power_of_5 = (decimal as f64).log(5.0).ceil() as i64 + 1;

    let mut vars = vec![];
    let mut sum_expr = z3::ast::Int::from_i64(&ctx, 0);

    for i in 0..=max_power_of_5 {
        let var = z3::ast::Int::new_const(&ctx, format!("power_{}", i));

        solver.assert(&var.ge(&z3::ast::Int::from_i64(&ctx, -2)));
        solver.assert(&var.le(&z3::ast::Int::from_i64(&ctx, 2)));

        sum_expr += &var * z3::ast::Int::from_i64(&ctx, 5_i64.pow(i as u32));

        vars.push(var);
    }

    solver.assert(&sum_expr._eq(&z3::ast::Int::from_i64(&ctx, decimal)));

    let mut result = String::new();

    if let z3::SatResult::Sat = solver.check() {
        let model = solver.get_model().unwrap();

        for v in vars.iter().rev() {
            let value = model.eval(v, true).unwrap().as_i64().unwrap();
            match value {
                0 => result.push('0'),
                1 => result.push('1'),
                2 => result.push('2'),
                -1 => result.push('-'),
                -2 => result.push('='),
                _ => unreachable!(),
            }
        }
    }

    result.chars().skip_while(|&x| x == '0').collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let sum = input
        .lines()
        .map(|line| snafu_to_decimal(line.to_string()))
        .sum();
    Some(decimal_to_snafu(sum))
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=-0-2".to_string()), 1747);
        assert_eq!(snafu_to_decimal("12111".to_string()), 906);
        assert_eq!(snafu_to_decimal("2=0=".to_string()), 198);
        assert_eq!(snafu_to_decimal("21".to_string()), 11);
        assert_eq!(snafu_to_decimal("2=01".to_string()), 201);
        assert_eq!(snafu_to_decimal("111".to_string()), 31);
        assert_eq!(snafu_to_decimal("20012".to_string()), 1257);
        assert_eq!(snafu_to_decimal("112".to_string()), 32);
        assert_eq!(snafu_to_decimal("1=-1=".to_string()), 353);
        assert_eq!(snafu_to_decimal("1-12".to_string()), 107);
        assert_eq!(snafu_to_decimal("12".to_string()), 7);
        assert_eq!(snafu_to_decimal("1=".to_string()), 3);
        assert_eq!(snafu_to_decimal("122".to_string()), 37);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1".to_string());
        assert_eq!(decimal_to_snafu(2), "2".to_string());
        assert_eq!(decimal_to_snafu(3), "1=".to_string());
        assert_eq!(decimal_to_snafu(4), "1-".to_string());
        assert_eq!(decimal_to_snafu(5), "10".to_string());
        assert_eq!(decimal_to_snafu(6), "11".to_string());
        assert_eq!(decimal_to_snafu(7), "12".to_string());
        assert_eq!(decimal_to_snafu(8), "2=".to_string());
        assert_eq!(decimal_to_snafu(9), "2-".to_string());
        assert_eq!(decimal_to_snafu(10), "20".to_string());
        assert_eq!(decimal_to_snafu(15), "1=0".to_string());
        assert_eq!(decimal_to_snafu(20), "1-0".to_string());
        assert_eq!(decimal_to_snafu(2022), "1=11-2".to_string());
        assert_eq!(decimal_to_snafu(12345), "1-0---0".to_string());
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0".to_string());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 25);
        assert_eq!(part_one(&input), Some("2011-=2=-1020-1===-1".to_string()));
        assert_eq!(part_two(&input), None);
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 25);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 25);
        b.iter(|| part_two(input));
    }
}
