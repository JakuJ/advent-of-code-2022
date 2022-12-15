#![feature(test)]

use advent_of_code::helpers::parse_with_regex;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type Point = (i32, i32);

const TARGET_Y: i32 = 2_000_000;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"^Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)$"
    )
    .unwrap();
}

fn parse_line(line: &str) -> (Point, Point) {
    let [sx, sy, bx, by] = parse_with_regex::<i32, 4>(&RE, line);
    ((sx, sy), (bx, by))
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<(Point, Point)> = input.lines().map(parse_line).collect();

    let manhattan =
        |a: Point, b: Point| (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1));

    let mut ranges = vec![];

    for &((sx, sy), (bx, by)) in data.iter() {
        let dist = manhattan((sx, sy), (bx, by));

        let vert_dist = sy.abs_diff(TARGET_Y) as i32;

        let slice_width = 2 * dist + 1; // at sy
        let slice_width = slice_width - 2 * vert_dist; // at TARGET_Y

        if slice_width > 0 {
            let slice_start = sx - dist + vert_dist;
            let slice_end = slice_start + slice_width - 1;
            ranges.push(slice_start..=slice_end);
        }
    }

    // merge overlapping ranges in a loop until no changes are made
    'again: loop {
        for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                if i == j {
                    continue;
                }

                let (a, b) = (&ranges[i], &ranges[j]);

                // if ranges overlap
                if a.start() <= b.end() && b.start() <= a.end() {
                    let new_range = *a.start().min(b.start())..=*b.end().max(a.end());
                    ranges.remove(j);
                    ranges.remove(i);
                    ranges.push(new_range);
                    continue 'again;
                }
            }
        }
        break;
    }

    // remove all beacons at TARGET_Y
    let mut removed = HashSet::new();
    for &(bx, by) in data.iter().map(|(_, b)| b) {
        if by == TARGET_Y {
            for r in ranges.iter() {
                if r.contains(&bx) {
                    removed.insert(bx);
                    break;
                }
            }
        }
    }

    // return sum of sizes of the ranges
    let ranges_size = ranges
        .iter()
        .map(|r| (*r.end() - *r.start()) as u32 + 1)
        .sum::<u32>();

    Some(ranges_size - removed.len() as u32)
}

const COORD_MAX: i64 = 4_000_000;
struct ZPoint<'ctx> {
    x: z3::ast::Int<'ctx>,
    y: z3::ast::Int<'ctx>,
}

struct PuzzleModel<'ctx> {
    ctx: &'ctx z3::Context,
    beacon: ZPoint<'ctx>,
    data: Vec<(ZPoint<'ctx>, ZPoint<'ctx>)>,
}

impl<'ctx> PuzzleModel<'ctx> {
    fn from_sensor_data(data: Vec<(Point, Point)>, ctx: &'ctx z3::Context) -> Self {
        let point_to_z3 = |(x, y), ctx| ZPoint {
            x: z3::ast::Int::from_i64(ctx, x as i64),
            y: z3::ast::Int::from_i64(ctx, y as i64),
        };

        PuzzleModel {
            ctx,
            beacon: ZPoint {
                x: z3::ast::Int::fresh_const(ctx, "beacon_x"),
                y: z3::ast::Int::fresh_const(ctx, "beacon_y"),
            },
            data: data
                .iter()
                .map(|&(s, b)| (point_to_z3(s, ctx), point_to_z3(b, ctx)))
                .collect(),
        }
    }

    fn absolute(&self, a: &z3::ast::Int<'ctx>) -> z3::ast::Int<'ctx> {
        z3::ast::Int::ge(a, &z3::ast::Int::from_i64(self.ctx, 0)).ite(a, &-a)
    }

    fn manhattan(&self, s: &ZPoint<'ctx>, b: &ZPoint<'ctx>) -> z3::ast::Int<'ctx> {
        self.absolute(&(&s.x - &b.x)) + self.absolute(&(&s.y - &b.y))
    }

    fn sensor(&self, s: &ZPoint<'ctx>, b: &ZPoint<'ctx>) -> z3::ast::Bool<'ctx> {
        self.manhattan(s, b).lt(&self.manhattan(s, &self.beacon))
    }

    fn constrain(&self, solver: &z3::Solver) {
        solver.assert(&self.beacon.x.ge(&z3::ast::Int::from_i64(self.ctx, 0)));
        solver.assert(
            &self
                .beacon
                .x
                .le(&z3::ast::Int::from_i64(self.ctx, COORD_MAX)),
        );

        solver.assert(&self.beacon.y.ge(&z3::ast::Int::from_i64(self.ctx, 0)));
        solver.assert(
            &self
                .beacon
                .y
                .le(&z3::ast::Int::from_i64(self.ctx, COORD_MAX)),
        );

        for (s, b) in self.data.iter() {
            solver.assert(&self.sensor(s, b));
        }
    }

    fn extract_model(&self, model: &z3::Model) -> (u64, u64) {
        let bx = model.eval(&self.beacon.x, true).unwrap().as_u64().unwrap();
        let by = model.eval(&self.beacon.y, true).unwrap().as_u64().unwrap();
        (bx, by)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let data: Vec<(Point, Point)> = input.lines().map(parse_line).collect();

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let puzzle_model = PuzzleModel::from_sensor_data(data, &ctx);
    let solver = z3::Solver::new(&ctx);

    puzzle_model.constrain(&solver);

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let (bx, by) = puzzle_model.extract_model(&model);
            Some(bx * 4_000_000 + by)
        }
        _ => None,
    }
}

fn main() {
    let _ = RE.is_match(""); // pre-load regex
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 15);
        assert_eq!(part_one(&input), Some(5511201));
        assert_eq!(part_two(&input), Some(11318723411840));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 15);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 15);
        b.iter(|| part_two(input));
    }
}
