#![feature(test)]

use std::cmp::{max, min};

type Point = (usize, usize);
type Path = Vec<Point>;
type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn prepare_grid(paths: Vec<Path>, include_floor: bool) -> (Grid, usize) {
    let height = paths
        .iter()
        .flat_map(|x| x.iter().map(|x| x.1))
        .max()
        .unwrap();

    // include row 0
    let mut height = height + 1;

    if include_floor {
        height += 2;
    }

    let (leftmost, rightmost) = paths
        .iter()
        .flat_map(|x| x.iter().map(|x| x.0))
        .fold((usize::MAX, usize::MIN), |acc, x| {
            (min(acc.0, x), max(acc.1, x))
        });

    let cone_width = 2 * height + 1;

    let leftmost = min(leftmost, 500 - cone_width);
    let rightmost = max(rightmost, 500 + cone_width);

    let width = rightmost - leftmost + 1;
    let x_offset = leftmost;

    // create grid
    let mut grid = vec![vec!['.'; width]; height];

    for path in paths {
        for (&(sx, sy), &(ex, ey)) in path.iter().zip(path.iter().skip(1)) {
            if sx == ex {
                let (sy, ey) = if sy < ey { (sy, ey) } else { (ey, sy) };
                for y in sy..=ey {
                    grid[y][sx - x_offset] = '#';
                }
            } else {
                let (sx, ex) = if sx < ex { (sx, ex) } else { (ex, sx) };
                grid[sy][sx - x_offset..=ex - x_offset].fill('#');
            }
        }
    }

    if include_floor {
        grid[height - 1].fill('#');
    }

    println!("Grid size: {} x {}", width, height);

    (grid, x_offset)
}

fn simulate_falling_sand(grid: &mut Grid, x_offset: usize) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let x_origin = 500 - x_offset;
    let mut settled = 0;

    'sim: loop {
        let mut current = (x_origin, 0);
        loop {
            // check if we are at the very bottom
            if current.1 + 1 == height {
                break 'sim;
            }
            // move down if possible
            else if current.1 + 1 < height && grid[current.1 + 1][current.0] == '.' {
                current.1 += 1;
            }
            // otherwise move diagonally down and left
            else if current.0 > 0
                && current.1 + 1 < height
                && grid[current.1 + 1][current.0 - 1] == '.'
            {
                current.0 -= 1;
                current.1 += 1;
            }
            // otherwise move diagonally down and right
            else if current.0 + 1 < width
                && current.1 + 1 < height
                && grid[current.1 + 1][current.0 + 1] == '.'
            {
                current.0 += 1;
                current.1 += 1;
            }
            // otherwise stop
            else {
                settled += 1;

                // if we are at origin, stop simulation
                if current == (x_origin, 0) {
                    break 'sim;
                }

                grid[current.1][current.0] = 'O';
                break;
            }
        }
    }

    settled
}

pub fn part_one(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let (mut grid, offset) = prepare_grid(paths, false);
    Some(simulate_falling_sand(&mut grid, offset))
}

pub fn part_two(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let (mut grid, offset) = prepare_grid(paths, true);
    Some(simulate_falling_sand(&mut grid, offset))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }

    #[test]
    fn test_solution() {
        let input = advent_of_code::read_file("inputs", 14);
        assert_eq!(part_one(&input), Some(793));
        assert_eq!(part_two(&input), Some(24166));
    }

    #[bench]
    fn bench_part_one(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 14);
        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut test::Bencher) {
        let input = &advent_of_code::read_file("inputs", 14);
        b.iter(|| part_two(input));
    }
}
